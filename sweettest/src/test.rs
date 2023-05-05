use std::time::SystemTime;
use maplit::hashset;

use holochain::sweettest::*;
use holochain::conductor::ConductorHandle;
use holo_hash::*;

use perf_testing_integrity_zome::*;
use perf_testing_zome::*;

use crate::DNA_FILEPATH;
use sweettest_utils::*;

///
pub async fn test(arg: String) {
   let now = SystemTime::now();

   // Admin API test
   if arg == "" {
      test_list_apps().await;
   }
   // // Pub Key
   // if arg == "all" || arg == "key" {
   //    test_pub_enc_key().await;
   // }
   // // Encryption
   // if arg == "all" || arg == "enc" {
   //    test_encryption().await;
   // }
   // Handle
   if arg == "all" || arg == "handle" {
      test_handle().await;
   }
   // 10
   if arg == "all" || arg == "10" {
      test_many_handle(10).await;
   }

   // 100
   if arg == "all" || arg == "100" {
      test_many_handle(100).await;
   }

   // // Mail via DM
   // if arg == "all" || arg == "mail" {
   //    test_mail_dm().await;
   // }
   // // Mail via DHT
   // if arg == "all" || arg == "pending" {
   //    test_mail_pending().await;
   // }
   // // File
   // if arg == "all" || arg == "file" {
   //    std::env::set_var("WASM_LOG", "WARN");
   //    test_file_dm().await;
   // }

   // Print elapsed
   match now.elapsed() {
      Ok(elapsed) => {
         // it prints '2'
         println!("\n *** Test(s) duration: {} secs", elapsed.as_secs());
      }
      Err(e) => {
         // an error occurred!
         println!("Error: {:?}", e);
      }
   }
}


///
pub async fn test_list_apps() {
   //observability::test_run().ok();

   println!("Loading DNA...");
   let dna = SweetDnaFile::from_bundle(std::path::Path::new(DNA_FILEPATH))
      .await
      .unwrap();

   println!("INSTALLING TWO APPS...");
   // Install two apps on the Conductor:
   // Both share a CellId in common, and also include a distinct CellId each.
   let mut conductor = SweetConductor::from_standard_config().await;
   let alex = SweetAgents::one(conductor.keystore()).await;
   let app1 = conductor
      .setup_app_for_agent("app1", alex.clone(), &[dna.clone()])
      .await
      .unwrap();
   let _app2 = conductor
      .setup_app_for_agent("app2", alex.clone(), &[dna])
      .await
      .unwrap();

   let cell1 = app1.into_cells()[0].clone();

   println!("\n LIST RUNNING APPS...");
   let list_apps = |conductor: ConductorHandle, cell: SweetCell| async move {
      conductor
         .list_running_apps_for_dependent_cell_id(cell.cell_id())
         .await
         .unwrap()
   };
   let res = list_apps(conductor.clone(), cell1.clone()).await;
   println!("list_apps = {:?}", res);

   // - Ensure that the first CellId is associated with both apps,
   //   and the other two are only associated with one app each.
   assert_eq!(res, hashset!["app1".to_string(), "app2".to_string()]);
}



// pub async add_handles(count: usize) {
//
// }


///
pub async fn test_handle() {
   let (conductor, _alex, cell1) = setup_1_conductor(DNA_FILEPATH).await;

   let name = format!("{}-{}", "alex", 1);
   let ah1: ActionHash = conductor.call(&cell1.zome("zPerfTesting"), "add_handle", name.to_string()).await;
   println!("ah1: {:?}", ah1);
   //tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
   let handle: String = conductor.call(&cell1.zome("zPerfTesting"), "get_handle", ah1).await;
   println!("handle: {:?}", handle);
   assert_eq!(name, handle);
}

///
pub async fn test_many_handle(n: usize) {
   let (conductor, _alex, cell1) = setup_1_conductor(DNA_FILEPATH).await;

   let mut ahs = Vec::new();
   
   // set N handles
   for i in 0..n {
      let name = format!("{}-{}", "alex", i);
      let ah: ActionHash = conductor.call(&cell1.zome("zPerfTesting"), "add_handle", name.to_string()).await;
      ahs.push(ah);
   }

   // Do some gets
   for _ in 0..10u32 {
      let _handle: String = conductor.call(&cell1.zome("zPerfTesting"), "get_handle", ahs[0].clone()).await;
   }
}

