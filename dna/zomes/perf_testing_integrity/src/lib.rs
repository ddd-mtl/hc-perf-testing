#![allow(non_upper_case_globals)]
#![allow(unused_doc_comments)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_attributes)]


//--------------------------------------------------------------------------------------------------

use hdi::prelude::*;


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
    pub value: String,
}

impl Handle {
    pub fn new(name: String) -> Self {
        Self {
            value: name,
        }
    }

    pub fn empty() -> Self {
        Self {
            value: String::new(),
        }
    }

    /// DEBUG
    pub fn dummy() -> Self {
        Self {
            value: "dummy".to_string(),
        }
    }
}
