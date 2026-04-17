use serde::{Deserialize, Serialize};
use crate::packer::{Bin, Item, PackResult, Dimension, AxisSize, Dimensional};

#[derive(Serialize)]
struct ItemOutput {
    shape_id: i32,
    name: String,
    x: f64,
    y: f64,
    z: f64,
    width: f64,
    height: f64,
    depth: f64,
}

#[derive(Serialize)]
struct SpaceOutput {
    x: f64,
    y: f64,
    z: f64,
    width: f64,
    height: f64,
    depth: f64,
}

#[derive(Serialize, Deserialize)]
struct ItemInput {
    shape_id: i32,
    name: String,
    #[serde(default)]
    x: f64,
    #[serde(default)]
    y: f64,
    #[serde(default)]
    z: f64,
    width: f64,
    height: f64,
    depth: f64,
    #[serde(default = "default_quantity")]
    quantity: i32,
}

// TODO: Refactor test data to use quantity, and remove this default value
fn default_quantity() -> i32 {
    1
}

#[derive(Serialize)]
struct PackingDataOutput {
    bin: Bin,
    items: Vec<ItemOutput>,
    unpacked_items: Vec<ItemOutput>,
    free_spaces: Vec<SpaceOutput>,
}

#[derive(Serialize, Deserialize)]
struct PackingDataInput {
    bin: Bin,
    items: Vec<ItemInput>,
}

pub fn parse_bin_json(json: &str) -> Result<(Bin, Vec<Item>), serde_json::Error> {
    let data: PackingDataInput = serde_json::from_str(json)?;

    let bin = data.bin;
    let items = expand_items(data.items);

    Ok((bin, items))
}

pub fn convert_bin_json(result: PackResult) -> Result<String, serde_json::Error> {
    let unpacked_items: Vec<ItemOutput> = result.unplaced.iter().map(|item| {
        let size = item.size_xyz();
        ItemOutput {
            shape_id: item.shape_id,
            name: item.name.clone(),
            x: item.position_xyz[0],
            y: item.position_xyz[1],
            z: item.position_xyz[2],
            width: size[0],
            height: size[1],
            depth: size[2],
        }
    }).collect();

    let placed_items: Vec<ItemOutput> = result.placed.iter().map(|item| {
        let size = item.size_xyz();
        ItemOutput {
            shape_id: item.shape_id,
            name: item.name.clone(),
            x: item.position_xyz[0],
            y: item.position_xyz[1],
            z: item.position_xyz[2],
            width: size[0],
            height: size[1],
            depth: size[2],
        }
    }).collect();

    let free_spaces_output: Vec<SpaceOutput> = result.free_spaces.iter().map(|space| {
        let xyz = space.size_xyz();
        SpaceOutput {
            x: space.position_xyz[0],
            y: space.position_xyz[1],
            z: space.position_xyz[2],
            width: xyz[0],
            height: xyz[1],
            depth: xyz[2],
        }
    }).collect();

    let packing_data = PackingDataOutput {
        bin: result.bin,
        items: placed_items,
        unpacked_items: unpacked_items,
        free_spaces: free_spaces_output,
    };

    serde_json::to_string(&packing_data)
}

pub fn write_bin_to_file(bin: &Bin, items: Vec<Item>, file_name: &str) -> std::io::Result<()> {
    let packing_data = PackingDataInput {
        bin: bin.clone(),
        items: group_items(items),
    };

    let json_data = serde_json::to_string_pretty(&packing_data)?;
    std::fs::write(file_name, json_data)
}

pub fn load_bin_from_file(file_name: &str) -> std::io::Result<(Bin, Vec<Item>)> {
    let json_data = std::fs::read_to_string(file_name)?;
    match parse_bin_json(&json_data) {
        Ok((bin, items)) => Ok((bin, items)),
        Err(e) => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e)),
    }
}

// Processes input items into a vector of items, with duplicates based on their specified quantity. This format is needed for the packing algo.
fn expand_items(input_items: Vec<ItemInput>) -> Vec<Item> {
    let mut items = Vec::new();
    for input_item in input_items {
        for _ in 0..input_item.quantity {
            items.push(Item {
                shape_id: input_item.shape_id,
                name: input_item.name.clone(),
                position_xyz: [input_item.x, input_item.y, input_item.z],
                size: [
                    Dimension { length: input_item.width, axis: AxisSize::Width },
                    Dimension { length: input_item.height, axis: AxisSize::Height },
                    Dimension { length: input_item.depth, axis: AxisSize::Depth },
                ],
            });
        }
    }
    items
}

// Processes items by collapsing same ids into one, returning ItemInput with quantity tracking.
fn group_items(items: Vec<Item>) -> Vec<ItemInput> {
    use std::collections::HashMap;

    let mut items_map: HashMap<i32, Vec<Item>> = HashMap::new();
    for item in items {
        items_map.entry(item.shape_id).or_insert_with(Vec::new).push(item);
    }

    items_map.into_iter().map(|(id, items)| {
        let qty = items.len() as i32;
        let item = &items[0];  // Use first item for dimensions
        let size = item.size_xyz();
        ItemInput {
            shape_id: id,
            name: item.name.clone(),
            x: item.position_xyz[0],
            y: item.position_xyz[1],
            z: item.position_xyz[2],
            width: size[0],
            height: size[1],
            depth: size[2],
            quantity: qty,
        }
    }).collect()
}