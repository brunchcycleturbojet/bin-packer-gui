use serde::{Deserialize, Serialize};
use packer_3d::{box3d::Box3D, sorting::Sorting, vector3d::Vector3D, PackerInstance};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn pack_bin(json: &str) -> String {
    
    // Create the packer instance from input JSON
    let mut packer = match parse_bin_json(json) {
        Ok(packer) => packer,
        Err(e) => {
            eprintln!("Error parsing JSON: {}", e);
            return String::new(); // Empty JSON response on error
        }
    };

    // Do packing using bin_packer_3d
    match packer.pack_all() {
        Err(errors) => println!("Errors: {:#?}", errors),
        Ok(()) => {}
	}

    // Generate response JSON
    let result_json = match convert_bin_json(packer) {
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

#[derive(Deserialize, Serialize)]
struct PackingData<'a> {
    bin: BinData,
    #[serde(borrow)]
    items: Vec<ItemData<'a>>,
}

#[derive(Deserialize, Serialize)]
struct BinData {
    width: u64,
    height: u64,
    depth: u64,
}

#[derive(Deserialize, Serialize)]
struct ItemData<'a> {
    id: usize,
    name: &'a str,
    x: u64, // Positional data is positive quadrants only, for use with packer_3d
    y: u64, // Ideally we'd just run the algo with f64 instead, but patching the library is a pain, and 
    z: u64, // the conversion overhead is small anyways. Size limit is acceptable too.
    width: u64,
    height: u64,
    depth: u64,
}

struct ItemMiscData<'a> {
    id: usize,
    name: &'a str,
    colour: &'a str,
}

fn parse_bin_json<'a>(json: &'a str) -> Result<PackerInstance, serde_json::Error> {
    let data: PackingData = serde_json::from_str(json)?;

    let items: Vec<Box3D> = data.items.into_iter().map(|item| 
        Box3D::from_xyz_whl(item.x, item.y, item.z, item.width, item.height, item.depth, item.id, 0)
    ).collect();

    let packer = PackerInstance::new(
        items,
        Vector3D::new(data.bin.width, data.bin.height, data.bin.depth),
        true,
        (false, false, false),
        Sorting::descending_volume
    );

    Ok(packer)
}

fn convert_bin_json<'a>(packer: PackerInstance) -> Result<String, serde_json::Error> {

    let bin: BinData = BinData { width: packer.container_size().x, height: packer.container_size().y, depth: packer.container_size().z };

    // Convert each Box3D into ItemData
    let items: Vec<ItemData> = packer.boxes().into_iter().map(|box_3d| ItemData {
        id: box_3d.id,
        name: "", // TODO: Save non packer_3d data and retrieve it here, matching on id
        x: box_3d.position.x,
        y: box_3d.position.y,
        z: box_3d.position.z,
        width: box_3d.size.x,
        height: box_3d.size.y,
        depth: box_3d.size.z,
    }).collect();

    let packing_data = PackingData {
        bin: bin,
        items: items,
    };

    serde_json::to_string(&packing_data)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![pack_bin])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
