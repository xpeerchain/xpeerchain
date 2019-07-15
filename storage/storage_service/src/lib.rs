// Copyright (c) The XPeer Core Contributors
// SPDX-License-Identifier: Apache-2.0

//! This crate implements the storage [GRPC](http://grpc.io) service.
//!
//! The user of storage service is supposed to use it via client lib provided in
//! [`storage_client`](../storage_client/index.html) instead of via
//! [`StorageClient`](../storage_proto/proto/storage_grpc/struct.StorageClient.html) directly.

pub mod mocks;

use config::config::NodeConfig;
use failure::prelude::*;
use grpc_helpers::{provide_grpc_response, spawn_service_thread_with_drop_closure, ServerHandle};
use xpeerdb::XPeerDB;
use logger::prelude::*;
use metrics::counters::SVC_COUNTERS;
use proto_conv::{FromProto, IntoProto};
use std::{
    ops::Deref,
    path::Path,
    sync::{mpsc, Arc, Mutex},
};
use storage_proto::proto::{
    storage::{
        GetAccountStateWithProofByStateRootRequest, GetAccountStateWithProofByStateRootResponse,
        GetExecutorStartupInfoRequest, GetExecutorStartupInfoResponse, GetTransactionsRequest,
        GetTransactionsResponse, SaveTransactionsRequest, SaveTransactionsResponse,
    },
    storage_grpc::{create_storage, Storage},
};
use types::proto::get_with_proof::{UpdateToLatestLedgerRequest, UpdateToLatestLedgerResponse};

/// Starts storage service according to config.
pub fn start_storage_service(config: &NodeConfig) -> ServerHandle {
    let (storage_service, shutdown_receiver) = StorageService::new(&config.storage.get_dir());
    spawn_service_thread_with_drop_closure(
        create_storage(storage_service),
        config.storage.address.clone(),
        config.storage.port,
        "storage",
        move || {
            shutdown_receiver
                .recv()
                .expect("Failed to receive on shutdown channel when storage service was dropped")
        },
    )
}

/// The implementation of the storage [GRPC](http://grpc.io) service.
///
/// It serves [`XPeerDB`] APIs over the network. See API documentation in [`storage_proto`] and
/// [`XPeerDB`].
#[derive(Clone)]
pub struct StorageService {
    db: Arc<XPeerDBWrapper>,
}

/// When dropping GRPC server we want to wait until XPeerDB is dropped first, so the RocksDB
/// instance held by GRPC threads is closed before the main function of GRPC server
/// finishes. Otherwise, if we don't manually guarantee this, some thread(s) may still be
/// alive holding an Arc pointer to XPeerDB after main function of GRPC server returns.
/// Having this wrapper with a channel gives us a way to signal the receiving end that all GRPC
/// server threads are joined so RocksDB is closed.
///
/// See these links for more details.
///   https://github.com/pingcap/grpc-rs/issues/227
///   https://github.com/facebook/rocksdb/issues/649
struct XPeerDBWrapper {
    db: Option<XPeerDB>,
    shutdown_sender: Mutex<mpsc::Sender<()>>,
}

impl XPeerDBWrapper {
    pub fn new<P: AsRef<Path>>(path: &P) -> (Self, mpsc::Receiver<()>) {
        let db = XPeerDB::new(path);
        let (shutdown_sender, shutdown_receiver) = mpsc::channel();
        (
            Self {
                db: Some(db),
                shutdown_sender: Mutex::new(shutdown_sender),
            },
            shutdown_receiver,
        )
    }
}

impl Drop for XPeerDBWrapper {
    fn drop(&mut self) {
        // Drop inner XPeerDB instance.
        self.db.take();
        // Send the shutdown message after DB is dropped.
        self.shutdown_sender
            .lock()
            .expect("Failed to lock mutex.")
            .send(())
            .expect("Failed to send shutdown message.");
    }
}

impl Deref for XPeerDBWrapper {
    type Target = XPeerDB;

    fn deref(&self) -> &Self::Target {
        self.db.as_ref().expect("XPeerDB is dropped unexptectedly")
    }
}

impl StorageService {
    /// This opens a [`XPeerDB`] at `path` and returns a [`StorageService`] instance serving it.
    ///
    /// A receiver side of a channel is also returned through which one can receive a notice after
    /// all resources used by the service including the underlying [`XPeerDB`] instance are
    /// fully dropped.
    ///
    /// example:
    /// ```no_run,
    ///    # use storage_service::*;
    ///    # use std::path::Path;
    ///    let (service, shutdown_receiver) = StorageService::new(&Path::new("path/to/db"));
    ///
    ///    drop(service);
    ///    shutdown_receiver.recv().expect("recv() should succeed.");
    ///
    ///    // XPeerDB instance is guaranteed to be properly dropped at this point.
    /// ```
    pub fn new<P: AsRef<Path>>(path: &P) -> (Self, mpsc::Receiver<()>) {
        let (db_wrapper, shutdown_receiver) = XPeerDBWrapper::new(path);
        (
            Self {
                db: Arc::new(db_wrapper),
            },
            shutdown_receiver,
        )
    }
}

impl StorageService {
    fn update_to_latest_ledger_inner(
        &self,
        req: UpdateToLatestLedgerRequest,
    ) -> Result<UpdateToLatestLedgerResponse> {
        let rust_req = types::get_with_proof::UpdateToLatestLedgerRequest::from_proto(req)?;

        let (response_items, ledger_info_with_sigs, validator_change_events) = self
            .db
            .update_to_latest_ledger(rust_req.client_known_version, rust_req.requested_items)?;

        let rust_resp = types::get_with_proof::UpdateToLatestLedgerResponse {
            response_items,
            ledger_info_with_sigs,
            validator_change_events,
        };

        Ok(rust_resp.into_proto())
    }

    fn get_transactions_inner(
        &self,
        req: GetTransactionsRequest,
    ) -> Result<GetTransactionsResponse> {
        let rust_req = storage_proto::GetTransactionsRequest::from_proto(req)?;

        let txn_list_with_proof = self.db.get_transactions(
            rust_req.start_version,
            rust_req.batch_size,
            rust_req.ledger_version,
            rust_req.fetch_events,
        )?;

        let rust_resp = storage_proto::GetTransactionsResponse::new(txn_list_with_proof);

        Ok(rust_resp.into_proto())
    }

    fn get_account_state_with_proof_by_state_root_inner(
        &self,
        req: GetAccountStateWithProofByStateRootRequest,
    ) -> Result<GetAccountStateWithProofByStateRootResponse> {
        let rust_req = storage_proto::GetAccountStateWithProofByStateRootRequest::from_proto(req)?;

        let (account_state_blob, sparse_merkle_proof) =
            self.db.get_account_state_with_proof_by_state_root(
                rust_req.address,
                rust_req.state_root_hash,
            )?;

        let rust_resp = storage_proto::GetAccountStateWithProofByStateRootResponse {
            account_state_blob,
            sparse_merkle_proof,
        };

        Ok(rust_resp.into_proto())
    }

    fn save_transactions_inner(
        &self,
        req: SaveTransactionsRequest,
    ) -> Result<SaveTransactionsResponse> {
        let rust_req = storage_proto::SaveTransactionsRequest::from_proto(req)?;
        self.db.save_transactions(
            &rust_req.txns_to_commit,
            rust_req.first_version,
            &rust_req.ledger_info_with_signatures,
        )?;
        Ok(SaveTransactionsResponse::new())
    }

    fn get_executor_startup_info_inner(&self) -> Result<GetExecutorStartupInfoResponse> {
        let info = self.db.get_executor_startup_info()?;
        let rust_resp = storage_proto::GetExecutorStartupInfoResponse { info };
        Ok(rust_resp.into_proto())
    }
}

impl Storage for StorageService {
    fn update_to_latest_ledger(
        &mut self,
        ctx: grpcio::RpcContext<'_>,
        req: UpdateToLatestLedgerRequest,
        sink: grpcio::UnarySink<UpdateToLatestLedgerResponse>,
    ) {
        debug!("[GRPC] Storage::update_to_latest_ledger");
        let _timer = SVC_COUNTERS.req(&ctx);
        let resp = self.update_to_latest_ledger_inner(req);
        provide_grpc_response(resp, ctx, sink);
    }

    fn get_transactions(
        &mut self,
        ctx: grpcio::RpcContext,
        req: GetTransactionsRequest,
        sink: grpcio::UnarySink<GetTransactionsResponse>,
    ) {
        debug!("[GRPC] Storage::get_transactions");
        let _timer = SVC_COUNTERS.req(&ctx);
        let resp = self.get_transactions_inner(req);
        provide_grpc_response(resp, ctx, sink);
    }

    fn get_account_state_with_proof_by_state_root(
        &mut self,
        ctx: grpcio::RpcContext,
        req: GetAccountStateWithProofByStateRootRequest,
        sink: grpcio::UnarySink<GetAccountStateWithProofByStateRootResponse>,
    ) {
        debug!("[GRPC] Storage::get_account_state_with_proof_by_state_root");
        let _timer = SVC_COUNTERS.req(&ctx);
        let resp = self.get_account_state_with_proof_by_state_root_inner(req);
        provide_grpc_response(resp, ctx, sink);
    }

    fn save_transactions(
        &mut self,
        ctx: grpcio::RpcContext,
        req: SaveTransactionsRequest,
        sink: grpcio::UnarySink<SaveTransactionsResponse>,
    ) {
        debug!("[GRPC] Storage::save_transactions");
        let _timer = SVC_COUNTERS.req(&ctx);
        let resp = self.save_transactions_inner(req);
        provide_grpc_response(resp, ctx, sink);
    }

    fn get_executor_startup_info(
        &mut self,
        ctx: grpcio::RpcContext,
        _req: GetExecutorStartupInfoRequest,
        sink: grpcio::UnarySink<GetExecutorStartupInfoResponse>,
    ) {
        debug!("[GRPC] Storage::get_executor_startup_info");
        let _timer = SVC_COUNTERS.req(&ctx);
        let resp = self.get_executor_startup_info_inner();
        provide_grpc_response(resp, ctx, sink);
    }
}

#[cfg(test)]
mod storage_service_test;