use serde::{Deserialize, Serialize};
use crate::packer::{Bin, Item, PackResult};

#[derive(Deserialize, Serialize)]
struct PackingDataOutput {
    bin: BinData,
    items: Vec<ItemData>,
    unpacked_items: Vec<ItemData>,
}

#[derive(Deserialize, Serialize)]
struct PackingData {
    bin: BinData,
    items: Vec<ItemData>,
}

#[derive(Deserialize, Serialize)]
struct BinData {
    width: f64,
    height: f64,
    depth: f64,
}

#[derive(Deserialize, Serialize)]
struct ItemData {
    id: usize,
    name: String,
    x: f64,
    y: f64,
    z: f64,
    width: f64,
    height: f64,
    depth: f64,
}

struct ItemMiscData {
    id: usize,
    name: String,
    colour: String,
}

pub fn parse_bin_json(json: &str) -> Result<(Bin, Vec<Item>), serde_json::Error> {
    let data: PackingData = serde_json::from_str(json)?;

    // Create the bin
    let bin = Bin::new(
        "bin".to_string(),
        data.bin.width,
        data.bin.height,
        data.bin.depth
    );

    // Create items
    let items: Vec<Item> = data.items.into_iter().enumerate().map(|(_index, item)| {
        Item {
            id: item.id as i32,
            name: item.name.to_string(),
            x: 0.0, // Will be set by packer
            y: 0.0,
            z: 0.0,
            width: item.width,
            height: item.height,
            depth: item.depth,
            rotate_x: 0.0, // Will be set by packer
            rotate_y: 0.0,
            rotate_z: 0.0,
        }
    }).collect();

    Ok((bin, items))
}

pub fn convert_bin_json(result: PackResult) -> Result<String, serde_json::Error> {
    let bin: BinData = BinData {
        width: result.bin.width,
        height: result.bin.height,
        depth: result.bin.depth
    };

    // Convert placed items to ItemData
    let items: Vec<ItemData> = result.placed.into_iter().map(|item| ItemData {
        id: item.id as usize,
        name: item.name,
        x: item.x,
        y: item.y,
        z: item.z,
        width: item.width,
        height: item.height,
        depth: item.depth,
    }).collect();

    // Convert unplaced items to ItemData (with zero positions)
    let unpacked_items: Vec<ItemData> = result.unplaced.into_iter().map(|item| ItemData {
        id: item.id as usize,
        name: item.name,
        x: 0.0,
        y: 0.0,
        z: 0.0,
        width: item.width,
        height: item.height,
        depth: item.depth,
    }).collect();

    let packing_data = PackingDataOutput {
        bin: bin,
        items: items,
        unpacked_items: unpacked_items,
    };

    serde_json::to_string(&packing_data)
}

// #[test]
// fn test_placement_within_bounds() {
//     let items = vec![
//         Geometry3D::new("B1a", 10.0, 10.0, 20.0).with_quantity(1), 
//         Geometry3D::new("B1b", 10.0, 10.0, 10.0).with_quantity(1), 
//         Geometry3D::new("B1c", 10.0, 10.0, 10.0).with_quantity(1), 
//         Geometry3D::new("B1d", 10.0, 10.0, 10.0).with_quantity(1), 
//         Geometry3D::new("B2", 60.0, 10.0, 10.0).with_quantity(1)];

//     let bin = Boundary3D::new(30.0, 10.0, 20.0);
//     let config = Config::default();
//     let packer = Packer3D::new(config);

//     let result = packer.solve(&items, &bin).unwrap();

//     println!("{:?}", bin.dimensions());
//     for placement in result.placements.iter() {
//         println!("{} {:?}", placement.geometry_id ,placement.to_transform_3d());
//     }

//     assert_eq!(result.placements.len(), 4); // Four boxes should be placed
//     assert_eq!(result.unplaced.len(), 1);   // Larger box should be unplaced

//     // Verify placements are within bounds
//     // TODO: Figure out where the origin of the box is (center? bottom left?)
//     for p in &result.placements {
//         assert!(p.position[0] <= bin.width());
//         assert!(p.position[1] <= bin.height());
//         assert!(p.position[2] <= bin.depth());
//     }
// }