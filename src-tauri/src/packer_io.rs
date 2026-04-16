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

#[derive(Serialize)]
struct SpaceOutput {
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
    free_spaces: Vec<SpaceOutput>,
}

#[derive(Deserialize)]
struct PackingDataInput {
    bin: Bin,
    items: Vec<ItemInput>,
    unpacked_items: Vec<ItemInput>,
}

pub fn parse_bin_json(json: &str) -> Result<(Bin, Vec<Item>, Vec<Item>), serde_json::Error> {
    let data: PackingDataInput = serde_json::from_str(json)?;

    let bin = data.bin;
    let items: Vec<Item> = data.items.into_iter().map(|input_item| {
        Item {
            id: input_item.id,
            name: input_item.name,
            position_xyz: [input_item.x, input_item.y, input_item.z],
            size: [
                Dimension { length: input_item.width, axis: AxisSize::Width },
                Dimension { length: input_item.height, axis: AxisSize::Height },
                Dimension { length: input_item.depth, axis: AxisSize::Depth },
            ],
        }
    }).collect();

    let unpacked_items = data.unpacked_items.into_iter().map(|input_item| {
        Item {
            id: input_item.id,
            name: input_item.name,
            position_xyz: [input_item.x, input_item.y, input_item.z],
            size: [
                Dimension { length: input_item.width, axis: AxisSize::Width },
                Dimension { length: input_item.height, axis: AxisSize::Height },
                Dimension { length: input_item.depth, axis: AxisSize::Depth },
            ],
        }
    }).collect();

    Ok((bin, items, unpacked_items))
}

pub fn convert_bin_json(result: PackResult) -> Result<String, serde_json::Error> {
    let unpacked_items: Vec<ItemOutput> = result.unplaced.iter().map(|item| {
        let size = item.size_xyz();
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
        let size = item.size_xyz();
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

pub fn write_bin_to_file(bin: &Bin, items: Vec<Item>, unpacked: Vec<Item>, file_name: &str) -> std::io::Result<()> {
    let packing_data = PackingDataOutput {
        bin: bin.clone(),
        items: items.into_iter().map(|item| {
            let size = item.size_xyz();
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
        }).collect(),
        unpacked_items: unpacked.into_iter().map(|item| {
            let size = item.size_xyz();
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
        }).collect(),
        free_spaces: vec![],
    };

    let json_data = serde_json::to_string_pretty(&packing_data)?;
    std::fs::write(file_name, json_data)
}

pub fn load_bin_from_file(file_name: &str) -> std::io::Result<(Bin, Vec<Item>, Vec<Item>)> {
    let json_data = std::fs::read_to_string(file_name)?;
    match parse_bin_json(&json_data) {
        Ok((bin, items, unpacked)) => Ok((bin, items, unpacked)),
        Err(e) => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e)),
    }
}