// Copyright (c) The XPeer Core Contributors
// SPDX-License-Identifier: Apache-2.0

use proptest::test_runner::TestRunner;
use std::{fmt, ops::Deref, str::FromStr};

pub mod commands;
pub mod fuzz_targets;

/// Implementation for a particular target of a fuzz operation.
pub trait FuzzTargetImpl: Sync + Send + fmt::Debug {
    /// The name of the fuzz target.
    fn name(&self) -> &'static str;

    /// A description for this target.
    fn description(&self) -> &'static str;

    /// Generate a new example for this target to store in the corpus.
    fn generate(&self, runner: &mut TestRunner) -> Vec<u8>;

    /// Fuzz the target with this data. The fuzzer tests for panics or OOMs with this method.
    fn fuzz(&self, data: &[u8]);
}

/// A fuzz target.
#[derive(Copy, Clone, Debug)]
pub struct FuzzTarget(&'static (dyn FuzzTargetImpl + 'static));

impl Deref for FuzzTarget {
    type Target = dyn FuzzTargetImpl + 'static;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl FromStr for FuzzTarget {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        FuzzTarget::by_name(s).ok_or_else(|| format!("Fuzz target '{}' not found (run `list`)", s))
    }
}
