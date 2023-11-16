#![warn(unreachable_pub)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![deny(unused_must_use, rust_2018_idioms)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(feature = "with-serde", not(feature = "serde")))]
compile_error!("`with-serde` feature has been renamed to `serde`.");

#[macro_use]
extern crate alloc;

pub mod db;
mod evm_factory;
mod context;
mod evm;
mod frame;
pub mod handler;
mod inspector;
mod journaled_state;

#[cfg(feature = "optimism")]
pub mod optimism;

pub type DummyStateDB = InMemoryDB;
#[cfg(feature = "std")]
pub use db::{
    CacheState, DBBox, State, StateBuilder, StateDBBox, TransitionAccount, TransitionState,
};
pub use db::{Database, DatabaseCommit, DatabaseRef, InMemoryDB};
pub use evm_factory::{new, EvmFactory};
pub use context::{EvmContext,Context};
pub use evm::{new_evm, Evm, Transact, CALL_STACK_LIMIT};
pub use frame::{CallStackFrame,FrameOrResult};
pub use journaled_state::{JournalCheckpoint, JournalEntry, JournaledState};

// reexport `revm_precompiles`
#[doc(inline)]
pub use revm_precompile as precompile;

// reexport `revm_interpreter`
#[doc(inline)]
pub use revm_interpreter as interpreter;

// reexport `revm_primitives`
#[doc(inline)]
pub use revm_interpreter::primitives;

// reexport inspector implementations
pub use inspector::inspectors;
pub use inspector::{inspector_instruction, Inspector};

// export Optimism types, helpers, and constants
#[cfg(feature = "optimism")]
pub use optimism::{L1BlockInfo, BASE_FEE_RECIPIENT, L1_BLOCK_CONTRACT, L1_FEE_RECIPIENT};

pub use handler::Handler;
