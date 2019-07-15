---
id: move-language
title: Move Language
custom_edit_url: https://github.com/xpeer/xpeer/edit/master/language/README.md
---

# Move

Move is a new programming language developed to provide a safe and programmable foundation for the XPeer Blockchain.

## Organization

The Move language directory consists of five parts:

- The [virtual machine](https://github.com/xpeer/xpeer/tree/master/language/vm) (VM), which contains the bytecode format, a bytecode interpreter, and infrastructure for executing a block of transactions. This directory also contains the infrastructure to generate the genesis block.

- The [bytecode verifier](https://github.com/xpeer/xpeer/tree/master/language/bytecode_verifier), which contains a static analysis tool for rejecting invalid Move bytecode. The virtual machine runs the bytecode verifier on any new Move code it encounters before executing it. The compiler runs the bytecode verifier on its output and surfaces the errors to the programmer.

- The Move intermediate representation (IR) [compiler](https://github.com/xpeer/xpeer/tree/master/language/stdlib), which compiles human-readable program text into Move bytecode. *Warning: the IR compiler is a testing tool. It can generate invalid bytecode that will be rejected by the Move bytecode verifier. The IR syntax is a work in progress that will undergo significant changes.*

- The [standard library](https://github.com/xpeer/xpeer/tree/master/language/stdlib), which contains the Move IR code for the core system modules such as `XPeerAccount` and `XPeerCoin`.

- The [tests](https://github.com/xpeer/xpeer/tree/master/language/functional_tests) for the virtual machine, bytecode verifier, and compiler. These tests are written in Move IR and run by a testing framework that parses the expected result of running a test from special directives encoded in comments.

## How the Move Language Fits Into XPeer Core

XPeer Core components interact with the language component through the VM. Specifically, the [admission control](https://github.com/xpeer/xpeer/tree/master/admission_control) component uses a limited, read-only [subset](https://github.com/xpeer/xpeer/tree/master/vm_validator) of the VM functionality to discard invalid transactions before they are admitted to the mempool and consensus. The [execution](https://github.com/xpeer/xpeer/tree/master/execution) component uses the VM to execute a block of transactions.

### Exploring Move IR

* You can find many small Move IR examples in the [tests](https://github.com/xpeer/xpeer/tree/master/language/functional_tests/tests/testsuite). The easiest way to experiment with Move IR is to create a new test in this directory and follow the instructions for running the tests.
* Some more substantial examples can be found in the [standard library](https://github.com/xpeer/xpeer/tree/master/language/stdlib/modules). The two most notable ones are [XPeerAccount.mvir](https://github.com/xpeer/xpeer/blob/master/language/stdlib/modules/xpeer_account.mvir), which implements accounts on the XPeer blockchain, and [XPeerCoin.mvir](https://github.com/xpeer/xpeer/blob/master/language/stdlib/modules/xpeer_coin.mvir), which implements XPeer coin.
* The four transaction scripts supported in the XPeer testnet are also in the standard library directiory. They are [peer-to-peer transfer](https://github.com/xpeer/xpeer/blob/master/language/stdlib/transaction_scripts/peer_to_peer_transfer.mvir), [account creation](https://github.com/xpeer/xpeer/blob/master/language/stdlib/transaction_scripts/create_account.mvir), [minting new XPeer](https://github.com/xpeer/xpeer/blob/master/language/stdlib/transaction_scripts/mint.mvir) (will only work for an account with proper privileges), and [key rotation](https://github.com/xpeer/xpeer/blob/master/language/stdlib/transaction_scripts/rotate_authentication_key.mvir).
* The most complete documentation of the Move IR syntax is the [grammar](https://github.com/xpeer/xpeer/blob/master/language/compiler/src/parser/mod.rs). You can also take a look at the [parser for the Move IR](https://github.com/xpeer/xpeer/blob/master/language/compiler/src/parser/syntax.lalrpop).
* Check out the [IR compiler README](https://github.com/xpeer/xpeer/blob/master/language/compiler/README.md) for more details on writing Move IR code.

### Directory Organization

```
├── README.md          # This README
├── bytecode_verifier  # The bytecode verifier
├── e2e_tests          # infrastructure and tests for the end-to-end flow
├── functional_tests   # Testing framework for the Move language
├── compiler           # The IR to Move bytecode compiler
├── stdlib             # Core Move modules and transaction scripts
├── test.sh            # Script for running all the language tests
└── vm
    ├── cost_synthesis # Cost synthesis for bytecode instructions
    ├── src            # Bytecode language definitions, serializer, and deserializer
    ├── tests          # VM tests
    ├── vm_genesis     # The genesis state creation, and blockchain genesis writeset
    └── vm_runtime     # The bytecode interpreter
```
