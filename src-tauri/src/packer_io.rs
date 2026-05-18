use serde::{Deserialize, Serialize};
use crate::{packer::{AxisSize, Bin, Dimension, Dimensional, Item, PackResult}};

#[derive(Serialize, Deserialize, Clone)]
pub struct ItemInput {
    shape_id: i32,
    name: String,
    width: f64,
    height: f64,
    depth: f64,
    quantity: i32,
}

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
struct PackingDataInput {
    bin: Bin,
    items: Vec<ItemInput>,
}

#[derive(Serialize)]
struct PackingDataOutput {
    pub bin: Bin,
    pub item_pos: Vec<ItemOutput>,
    pub free_space_pos: Vec<SpaceOutput>,

    pub placed_items: Vec<ItemInput>,
    pub unpacked_items: Vec<ItemInput>,
}

#[derive(Serialize)]
struct LoadOutput {
    pack_input: PackingDataInput,
    pack_result: PackingDataOutput,
}

pub enum ResponseType {
    PackResult,
    LoadResult,
}

// Convert packing results into an appropriate JSON response
pub fn bin_to_json_response(result: PackResult, inputs: Vec<ItemInput>, response: ResponseType) -> Result<String, serde_json::Error> {
    match response {
        ResponseType::PackResult => serde_json::to_string(&process_results_to_output(result, inputs)),
        ResponseType::LoadResult => {
            let output = LoadOutput {
                pack_input: PackingDataInput {
                    bin: result.bin.clone(),
                    items: inputs.clone(),
                },
                pack_result: process_results_to_output(result, inputs),
            };
            serde_json::to_string(&output)
        }
    }
}

// Convert JSON pack request into bin and items data structures for packing, and the item inputs.
pub fn json_to_bin(json: &str) -> Result<(Bin, Vec<Item>, Vec<ItemInput>), serde_json::Error> {
    let data: PackingDataInput = serde_json::from_str(json)?;

    let bin = data.bin;
    let items = expand_items(data.items.clone());
    let item_inputs = data.items;
    Ok((bin, items, item_inputs))
}

// Write packing algo inputs to file
pub fn bin_to_file(bin: &Bin, items: Vec<Item>, file_name: &str) -> std::io::Result<()> {
    let packing_data = PackingDataInput {
        bin: bin.clone(),
        items: group_items(items),
    };

    let json_data = serde_json::to_string_pretty(&packing_data)?; // Use pretty for human readability in saved files
    std::fs::write(file_name, json_data)
}

// Read packing algo inputs from file into bin and items data structures, and the item inputs.
pub fn file_to_bin(file_path: &str) -> Result<(Bin, Vec<Item>, Vec<ItemInput>), std::io::Error> {
    let input_json = match std::fs::read_to_string(file_path) {
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return Err(e);
        },
        Ok(content) => content,
    };

    json_to_bin(&input_json).map_err(|e| {
        eprintln!("Error parsing JSON from file: {}", e);
        std::io::Error::new(std::io::ErrorKind::InvalidData, "Failed to parse JSON")
    })
}


// Helper functions --- 

fn process_results_to_output(result: PackResult, inputs: Vec<ItemInput>) -> PackingDataOutput {
    // Restore original dimensions for packed items, since the packer logic may have rotated them
    let mut placed_items: Vec<ItemInput> = group_items(result.placed.clone());
    placed_items.iter_mut().for_each(|item| {
        if let Some(input) = inputs.iter().find(|input| input.shape_id == item.shape_id) {
            item.width = input.width;
            item.height = input.height;
            item.depth = input.depth;
        }
    });

    let unpacked_items: Vec<ItemInput> = group_items(result.unplaced);

    let placed_item_positions: Vec<ItemOutput> = result.placed.iter().map(|item| {
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

    PackingDataOutput {
        bin: result.bin,
        item_pos: placed_item_positions,
        free_space_pos: free_spaces_output,
        placed_items: placed_items,
        unpacked_items: unpacked_items,
    }
}

// Processes input items into a vector of items, with duplicates based on their specified quantity. 
// This format is better for the packing algo so it can rotate and move items independently.
fn expand_items(input_items: Vec<ItemInput>) -> Vec<Item> {
    let mut items = Vec::new();
    for input_item in input_items {
        for _ in 0..input_item.quantity {
            items.push(Item {
                shape_id: input_item.shape_id,
                name: input_item.name.clone(),
                position_xyz: [0.0, 0.0, 0.0],
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

// Processes items into a vector of ItemInput, grouping same-shape items together.
// This format is better for UI display. Note that grouped items will lose their
// positions and dimensional 'rotations'.
fn group_items(items: Vec<Item>) -> Vec<ItemInput> {
    use indexmap::IndexMap;

    let mut items_map: IndexMap<i32, Vec<Item>> = IndexMap::new();
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
            width: size[0],
            height: size[1],
            depth: size[2],
            quantity: qty,
        }
    }).collect()
}