use serde::{Deserialize, Serialize};
use crate::packer::{Bin, Item, BinPacker3D, PackResult};

use crate::packer_io::{convert_bin_json, parse_bin_json};
mod packer_io;

mod packer;

#[tauri::command]
fn pack_bin(json: &str) -> String {

    // Parse the input JSON
    let (bin, items) = match parse_bin_json(json) {
        Ok((bin, items)) => (bin, items),
        Err(e) => {
            eprintln!("Error parsing JSON: {}", e);
            return String::new(); // Empty JSON response on error
        }
    };

    // Do packing
    let result = BinPacker3D::pack(bin, items);

    println!("Container: {}x{}x{}", result.bin.width, result.bin.height, result.bin.depth);
    println!("Packed {} items, {} items could not be packed", result.placed.len(), result.unplaced.len());

    println!("Packed items:");
    for item in &result.placed {
        println!("Item {}: position ({}, {}, {}), size ({}, {}, {})",
                 item.id, item.x, item.y, item.z, item.width, item.height, item.depth);
    }

    if !result.unplaced.is_empty() {
        println!("Unpacked items:");
        for item in &result.unplaced {
            println!("Item {}: size ({}, {}, {})",
                     item.id, item.width, item.height, item.depth);
        }
    }

    // Generate response JSON
    let result_json = match convert_bin_json(result) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("Error generating JSON: {}", e);
            return String::new(); // Empty JSON response on error
        }
    };

    return result_json;
}

#[tauri::command]
fn save_bin_and_items(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn load_bin_and_items(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![pack_bin])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
