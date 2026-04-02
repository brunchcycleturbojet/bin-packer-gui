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


    // let bin = Bin {
    //     width: 30.0,
    //     height: 30.0,
    //     depth: 30.0,
    // };

    // let mut items: Vec<Item> = Vec::new();
    // let mut item_id = 0;


    // for _ in 0..2 {
    //     items.push(Item {
    //         id: item_id,
    //         name: "small".to_string(),
    //         x: 0.0,
    //         y: 0.0,
    //         z: 0.0,
    //         width: 28.0,
    //         height: 28.0,
    //         depth: 2.0,
    //     });
    //     item_id += 1;
    // }

    // // for _ in 0..334 {
    // //     items.push(Item {
    // //         id: item_id,
    // //         name: "medium".to_string(),
    // //         x: 0.0,
    // //         y: 0.0,
    // //         z: 0.0,
    // //         width: 3.0,
    // //         height: 3.0,
    // //         depth: 3.0,
    // //     });
    // //     item_id += 1;
    // // }

    // for _ in 0..343 {
    //     items.push(Item {
    //         id: item_id,
    //         name: "large".to_string(),
    //         x: 0.0,
    //         y: 0.0,
    //         z: 0.0,
    //         width: 4.0,
    //         height: 4.0,
    //         depth: 4.0,
    //     });
    //     item_id += 1;
    // }

    // // Pack the items into the bin
    // let result = BinPacker3D::pack(bin, items);

    println!("Container: {}x{}x{}", result.bin.width, result.bin.height, result.bin.depth);
    println!("Time taken to pack: {} ms", result.time_to_pack);
    println!("Bin usage percentage: {:.2}%", result.bin_usage_percentage);
    println!("Packed {} items, {} items could not be packed", result.placed.len(), result.unplaced.len());
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
