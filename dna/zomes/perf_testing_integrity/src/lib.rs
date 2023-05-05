#![allow(non_upper_case_globals)]
#![allow(unused_doc_comments)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_attributes)]

//--------------------------------------------------------------------------------------------------

use hdi::prelude::*;
use hdi::prelude::holo_hash::hash_type;

pub use beads::*;
pub use query_log::*;
pub use globals::*;


#[hdk_entry_defs]
#[unit_enum(PerfTestingEntryTypes)]
pub enum PerfTestingEntry {
    #[entry_def(required_validations = 3, visibility = "public")]
    Handle(Handle),
    #[entry_def(required_validations = 1, visibility = "private")]
    PrivHandle(Handle),
}


/// List of all link kinds handled by this Zome
#[hdk_link_types]
#[derive(Serialize, Deserialize)]
pub enum PerfTestingLinkType {
    Any,
}



/// Entry representing the username of an Agent
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct Handle {
    pub username: String,
}

impl Handle {
    pub fn new(name: String) -> Self {
        Self {
            username: name,
        }
    }

    pub fn empty() -> Self {
        Self {
            username: String::new(),
        }
    }

    /// DEBUG
    pub fn dummy() -> Self {
        Self {
            username: "dummy".to_string(),
        }
    }
}
