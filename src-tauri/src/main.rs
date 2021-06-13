#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

mod ilias;
mod cmd;

use ilias::api::IliasApi;
use crate::cmd::*;


fn main() {
    // match IliasApi::new() {
    //     Ok(ilias_api) => {
    //         tauri::Builder::default()
    //             .manage(ilias_api)
    //             .invoke_handler(tauri::generate_handler![login, sync, open])
    //             .run(tauri::generate_context!())
    //             .expect("error while running tauri application");
    //     }
    //     Err(e) => {
    //         panic!("{}", e.to_string())
    //     }
    // }
}
