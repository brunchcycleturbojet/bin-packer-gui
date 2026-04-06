use serde::{Deserialize, Serialize};
use crate::packer::{Bin, Item, PackResult, Dimension, AxisSize, Dimensional};

#[derive(Serialize)]
struct ItemOutput {
    id: i32,
    name: String,
    x: f64,
    y: f64,
    z: f64,
    width: f64,
    height: f64,
    depth: f64,
}

#[derive(Deserialize)]
struct ItemInput {
    id: i32,
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
}

#[derive(Serialize)]
struct PackingDataOutput {
    bin: Bin,
    items: Vec<ItemOutput>,
    unpacked_items: Vec<ItemOutput>,
}

#[derive(Deserialize)]
struct PackingDataInput {
    bin: Bin,
    items: Vec<ItemInput>,
}

pub fn parse_bin_json(json: &str) -> Result<(Bin, Vec<Item>), serde_json::Error> {
    let data: PackingDataInput = serde_json::from_str(json)?;

    let bin = data.bin;
    let items: Vec<Item> = data.items.into_iter().map(|input_item| {
        // Ignore position and rotation data since we'll be calculating that shortly.
        Item {
            id: input_item.id,
            name: input_item.name,
            position_xyz: [0.0, 0.0, 0.0],
            size: [
                Dimension { length: input_item.width, axis: AxisSize::Width },
                Dimension { length: input_item.height, axis: AxisSize::Height },
                Dimension { length: input_item.depth, axis: AxisSize::Depth },
            ],
        }
    }).collect();

    Ok((bin, items))
}

pub fn convert_bin_json(result: PackResult) -> Result<String, serde_json::Error> {
    let unpacked_items: Vec<ItemOutput> = result.unplaced.iter().map(|item| {
        let size = item.as_xyz();
        ItemOutput {
            id: item.id,
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
        let size = item.as_xyz();
        ItemOutput {
            id: item.id,
            name: item.name.clone(),
            x: item.position_xyz[0],
            y: item.position_xyz[1],
            z: item.position_xyz[2],
            width: size[0],
            height: size[1],
            depth: size[2],
        }
    }).collect();

    let packing_data = PackingDataOutput {
        bin: result.bin,
        items: placed_items,
        unpacked_items: unpacked_items,
    };

    serde_json::to_string(&packing_data)
}
