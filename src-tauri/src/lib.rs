use crate::packer::{ BinPacker3D };
use crate::packer_io::{ResponseType, bin_to_file, bin_to_json_response, file_to_bin, json_to_bin};
mod packer_io;
mod packer;

fn run_packer(bin: packer::Bin, items: Vec<packer::Item>) -> packer::PackResult {
    let result = BinPacker3D::pack(bin, items);

    println!("--- Packing Result ---");
    println!("Container: {}x{}x{}", result.bin.width, result.bin.height, result.bin.depth);
    println!("Time taken to pack: {} ms", result.time_to_pack);
    println!("Bin usage percentage: {:.2}%", result.bin_usage_percentage);
    println!("Packed {} items, {} items could not be packed", result.placed.len(), result.unplaced.len());

    result
}

#[tauri::command]
fn pack_bin(json: &str) -> String {

    // Parse the input JSON
    let (bin, items, inputs) = match json_to_bin(json) {
        Ok((bin, items, inputs)) => (bin, items, inputs),
        Err(e) => {
            eprintln!("Error parsing JSON: {}", e);
            return String::new(); // Empty JSON response on error
        }
    };

    // Do packing
    let result = run_packer(bin, items);

    // Generate response JSON
    let result_json = match bin_to_json_response(result, inputs, ResponseType::PackResult) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("Error generating JSON: {}", e);
            return String::new(); // Empty JSON response on error
        }
    };

    result_json
}

#[tauri::command]
fn save_bin_and_items(json: &str, file_path: &str) -> String {

    // Parse the input JSON
    let (bin, items, _inputs) = match json_to_bin(json) {
        Ok((bin, items, inputs)) => (bin, items, inputs),
        Err(e) => {
            eprintln!("Error parsing JSON: {}", e);
            return String::new(); // Empty response on error
        }
    };

    match bin_to_file(&bin, items, file_path) {
        Ok(_) => file_path.to_string(),
        Err(e) => {
            eprintln!("Error writing to file: {}", e);
            String::new() // Empty response on error
        }
    }
}

#[tauri::command]
fn load_bin_and_items(file_path: &str) -> String {

    // Parse the input JSON
    let (bin, items, inputs) = match file_to_bin(file_path) {
        Ok((bin, items, inputs)) => (bin, items, inputs),
        Err(e) => {
            eprintln!("Error loading file: {}", e);
            return String::new(); // Empty JSON response on error
        }
    };
    
    // Do packing
    let result = run_packer(bin, items);

    // Generate response JSON
    let result_json = match bin_to_json_response(result, inputs, ResponseType::LoadResult) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("Error generating JSON: {}", e);
            return String::new(); // Empty JSON response on error
        }
    };

    result_json
}

#[cfg(test)]
mod packer_test;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![pack_bin, save_bin_and_items, load_bin_and_items])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
