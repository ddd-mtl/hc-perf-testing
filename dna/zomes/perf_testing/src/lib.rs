use hdk::prelude::*;
#[allow(unused_imports)]
use perf_testing_integrity::*;


///
#[hdk_extern]
pub fn get_handle(ah: ActionHash) -> ExternResult<String> {
   let fn_start = sys_time()?;
   let get_res = get(ah.clone(), GetOptions::content())?;
   let Some(record) = get_res
      else { return Err(wasm_error!("get_handle(): Entry not found")) };
   let record::RecordEntry::Present(entry) = record.entry()
      else { return Err(wasm_error!("Could not convert record")); };
   let Ok(res) = Handle::try_from(entry)
      else { return Err(wasm_error!("Could not convert entry")); };
   let fn_end = sys_time()?;
   debug!("GET TIME: {:?} ms", (fn_end.0 - fn_start.0) / 1000);
   Ok(res.value)
}




/// Zome Function
#[hdk_extern]
pub fn add_handle(new_username: String) -> ExternResult<ActionHash> {
   let fn_start = sys_time()?;
   // -- Create Handle Entry
   let new_handle = Handle::new(new_username.to_string());
   //let new_handle_eh = hash_entry(&new_handle)?;
   let new_handle_ah = create_entry(PerfTestingEntry::Handle(new_handle))?;
   //debug!("**** Handle linked to agent!");
   let fn_end = sys_time()?;
   debug!("ADD TIME: {:?} ms", (fn_end.0 - fn_start.0) / 1000);
   // Done
   return Ok(new_handle_ah);
}



/// Zome Function
#[hdk_extern]
pub fn add_linked_handle(new_username: String) -> ExternResult<ActionHash> {
   // -- Create Handle Entry
   let new_handle = Handle::new(new_username.to_string());
   let new_handle_eh = hash_entry(&new_handle)?;
   let new_handle_ah = create_entry(PerfTestingEntry::Handle(new_handle))?;
   // Create link
   let _ = create_link(
      Path::from("all_handles")
         .typed(PerfTestingLinkType::Any)?.path_entry_hash()?,
      new_handle_eh.clone(),
      PerfTestingLinkType::Any,
      LinkTag::from(()),
   )?;
   //debug!("**** Handle linked to agent!");
   // Done
   return Ok(new_handle_ah);
}
