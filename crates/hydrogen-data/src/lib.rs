//! Persistance et adaptateurs de stockage pour les sauvegardes de simulation Hydrogen.

pub mod error;
pub mod fs;
pub mod memory;
pub mod snapshot;
pub mod store;

pub use error::DataError;
pub use fs::FsStore;
pub use memory::MemoryStore;
pub use snapshot::SaveSnapshot;
pub use store::{validate_slot_name, SaveStore};
