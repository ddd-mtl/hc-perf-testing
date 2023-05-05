use hdk::prelude::*;
#[allow(unused_imports)]
use perf_testing_integrity::*;

/// Zome Function
#[hdk_extern]
pub fn get_handle(agent_id: ActionHash) -> ExternResult<String> {
   let maybe_current_handle = get_handle_element(agent_id);
   let str = match maybe_current_handle {
      None => "<noname>".to_string(),
      Some((handle, _ah)) => handle.username,
   };
   Ok(str)
}



/// Zome Function
#[hdk_extern]
pub fn add_handle(new_username: String) -> ExternResult<ActionHash> {
   /// -- Create Handle Entry
   let new_handle = Handle::new(new_username.to_string());
   //let new_handle_eh = hash_entry(&new_handle)?;
   let new_handle_ah = create_entry(SnapmailEntry::Handle(new_handle))?;
   //debug!("**** Handle linked to agent!");
   /// Done
   return Ok(new_handle_ah);
}



/// Zome Function
#[hdk_extern]
pub fn add_linked_handle(new_username: String) -> ExternResult<ActionHash> {
   /// -- Create Handle Entry
   let new_handle = Handle::new(new_username.to_string());
   let new_handle_eh = hash_entry(&new_handle)?;
   let new_handle_ah = create_entry(SnapmailEntry::Handle(new_handle))?;
   /// Create link
   let _ = create_link(
      Path::from("all_handles").typed(PerfTestingLinkType::Any).path_entry_hash()?,
      new_handle_eh.clone(),
      PerfTestingLinkType::Any,
      LinkTag::from(()),
   )?;
   //debug!("**** Handle linked to agent!");
   /// Done
   return Ok(new_handle_ah);
}
