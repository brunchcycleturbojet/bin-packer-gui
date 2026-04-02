use serde::{Deserialize, Serialize};
use crate::packer::{Bin, Item, PackResult};

#[derive(Deserialize, Serialize)]
struct PackingDataOutput {
    bin: Bin,
    items: Vec<Item>,
    unpacked_items: Vec<Item>,
}

#[derive(Deserialize, Serialize)]
struct PackingData {
    bin: Bin,
    items: Vec<Item>,
}

pub fn parse_bin_json(json: &str) -> Result<(Bin, Vec<Item>), serde_json::Error> {
    let data: PackingData = serde_json::from_str(json)?;

    let bin = data.bin;
    let items: Vec<Item> = data.items.into_iter().map(|mut item| {
        // Ignore position and rotation data since we'll be calculating that shortly.
        item.x = 0.0;
        item.y = 0.0;
        item.z = 0.0;
        item.rotate_x = 0.0; 
        item.rotate_y = 0.0;
        item.rotate_z = 0.0;
        item
    }).collect();

    Ok((bin, items))
}

pub fn convert_bin_json(result: PackResult) -> Result<String, serde_json::Error> {
    // For unplaced items, create with zero positions
    let unpacked_items: Vec<Item> = result.unplaced.into_iter().map(|item| Item {
        id: item.id,
        name: item.name,
        x: 0.0,
        y: 0.0,
        z: 0.0,
        width: item.width,
        height: item.height,
        depth: item.depth,
        rotate_x: 0.0,
        rotate_y: 0.0,
        rotate_z: 0.0,
    }).collect();

    let packing_data = PackingDataOutput {
        bin: result.bin,
        items: result.placed,
        unpacked_items,
    };

    serde_json::to_string(&packing_data)
}
