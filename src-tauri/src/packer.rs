use std::time::Instant;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Bin {
    pub width: f64,
    pub height: f64,
    pub depth: f64,
}

#[derive(Clone, Debug)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub position_xyz: [f64; 3],
    pub size: [Dimension; 3],
}
impl Dimensional for Item {
    fn get_size(&self) -> &[Dimension; 3] {
        &self.size
    }
}

#[derive(Clone, Debug)]
struct Space {
    position_xyz: [f64; 3], // x, y, z
    size: [Dimension; 3], // Unordered width/height/depth
}
impl Dimensional for Space {
    fn get_size(&self) -> &[Dimension; 3] {
        &self.size
    }
}

pub trait Dimensional {
    fn get_size(&self) -> &[Dimension; 3];

    fn volume(&self) -> f64 {
        let size = self.get_size();
        size[0].length * size[1].length * size[2].length
    }

    fn as_xyz(&self) -> [f64; 3] {
        let mut xyz = [0.0, 0.0, 0.0];
        for dim in self.get_size().iter() {
            xyz[dim.axis] = dim.length;
        }
        xyz
    }
}

#[derive(Clone, Debug)]
pub struct Dimension {
    pub length: f64,
    pub axis: AxisSize
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AxisSize { // Order defines coordinate convention (XYZ)
    Width = 0,
    Height = 1,
    Depth = 2
}
impl From<AxisSize> for usize {
    fn from(axis: AxisSize) -> Self {
        axis as usize
    }
}
impl<T> std::ops::Index<AxisSize> for [T; 3] {
    type Output = T;

    fn index(&self, axis: AxisSize) -> &T {
        &self[usize::from(axis)]
    }
}
impl<T> std::ops::IndexMut<AxisSize> for [T; 3] {
    fn index_mut(&mut self, axis: AxisSize) -> &mut T {
        let idx = usize::from(axis);
        &mut self[idx]
    }
}

pub struct PackResult {
    pub bin: Bin,
    pub placed: Vec<Item>,
    pub unplaced: Vec<Item>,
    pub time_to_pack: u128,
    pub bin_usage_percentage: f64,
}

pub struct BinPacker3D;

// Heuristic 3D bin packing for one rectangular bin and items, with axis-aligned rotations only.
// An optimal solution is NOT guaranteed. The algorithms in use are approximations as the problem is NP-hard (as of writing!).
// Based on the Shotput algorithm: https://medium.com/the-chain/solving-the-box-selection-algorithm-8695df087a4
//
// Bin/Item origin is considered at the bottom left corner.
// Coordinates are such that X = width, Y = height (up), Z = depth.
impl BinPacker3D {

    // Packs items into one bin.
    // Returns a copy of the input bin, placed items with sorted position/rotations, and any unplaced items.
    pub fn pack(bin: Bin, items: Vec<Item>) -> PackResult {
        let start_time = Instant::now();
        let mut unplaced = Vec::new();
        let mut placed = Vec::new();
        let mut free_spaces = vec![Space {
            position_xyz: [0.0, 0.0, 0.0],
            size: [
                Dimension { length: bin.width, axis: AxisSize::Width },
                Dimension { length: bin.height, axis: AxisSize::Height },
                Dimension { length: bin.depth, axis: AxisSize::Depth },
            ],
        }];

        // Sort items by largest dimension, descending - This will be the order we process items in.
        let mut sorted_items = items;

        // Move any items that are larger than the bin dimensions to the unplaced list first, since they can never be placed.
        // Although they might fit diagonally in a better arrangement, for simplicity we only try 90 degree rotations.
        sorted_items.retain(|item| {
            let item_max = item.size[0].length.max(item.size[1].length).max(item.size[2].length);
            let bin_max = bin.width.max(bin.height).max(bin.depth);
            if item_max > bin_max {
                unplaced.push(item.clone());
                false
            } else {
                true
            }
        });
        sorted_items.sort_by(|a, b| // Largest volume second
            b.volume().partial_cmp(&a.volume()).unwrap()
        );
        sorted_items.sort_by(|a, b| { // Largest dimension first
            let max_dim_a = a.size[0].length.max(a.size[1].length).max(a.size[2].length);
            let max_dim_b = b.size[0].length.max(b.size[1].length).max(b.size[2].length);
            max_dim_b.partial_cmp(&max_dim_a).unwrap()
        });

        for item in sorted_items {
            let mut best_fit: Option<(usize, [Dimension; 3], Space, Vec<Space>)> = None;

            // Find a space to fit the item
            for (index, space) in free_spaces.iter().enumerate() {

                if fits(space, &item) {
                    let (orientation, remainder) = Self::best_orientation(space, &item);

                    best_fit = Some((index, orientation, space.clone(), remainder));
                    break;
                }
            }

            if let Some((space_index, orientation, space, remainder)) = best_fit {
                // Space found, place the item and consume the space
                let mut placed_item = item.clone();
                placed_item.position_xyz = space.position_xyz.clone();
                placed_item.size = orientation;

                placed.push(placed_item);
                free_spaces.remove(space_index);

                // Update space blocks, make sure they're arrange from smallest volume to largest for first-fit
                free_spaces = [free_spaces, remainder].concat();
                free_spaces.sort_by(|a, b| {
                    a.volume().partial_cmp(&b.volume()).unwrap()
                });

            } else { 
                // No possible spaces found! Try the next item.
                unplaced.push(item);
            }
        }

        // Calculate metrics
        let time_to_pack = start_time.elapsed().as_millis();
        let bin_volume = bin.width * bin.height * bin.depth;
        let used_volume: f64 = placed.iter().map(|item| item.volume()).sum();
        let bin_usage_percentage = (used_volume / bin_volume) * 100.0;

        PackResult {
            bin,
            placed,
            unplaced,
            time_to_pack,
            bin_usage_percentage,
        }
    }

    fn best_orientation(space: &Space, item: &Item) -> ([Dimension; 3], Vec<Space>) {
        let space_xyz_dims = space.as_xyz();
        let item_xyz_dims = item.as_xyz();
        let mut b_dims = space.size.clone();
        let mut item_dims = item.size.clone();
        let mut remainder_blocks: Vec<Space> = Vec::new();

        // Sort dimensions ascending (shortest first)
        b_dims.sort_by(|a, b| a.length.partial_cmp(&b.length).unwrap());
        item_dims.sort_by(|a, b| a.length.partial_cmp(&b.length).unwrap());

        // Build the orientation of the item, side by side
        // First pass: Choose the shortest side of the box we can stack the item twice on its longest side,
        // Otherwise, try for an exact fit between the box and item dims
        let mut side_1: Option<usize> = None;
        for (i, b_dim) in b_dims.iter().enumerate() {
            if b_dim.length >= item_dims[2].length * 2.0 {
                side_1 = Some(i);

                // Create remainder block along fitted axis, sized to the item
                let mut xyz = space.position_xyz.clone();
                let mut size = item_xyz_dims.clone();
                xyz[b_dim.axis] += item_dims[2].length;
                size[b_dim.axis] = space_xyz_dims[b_dim.axis] - item_dims[2].length;

                remainder_blocks.push(Space {
                    position_xyz: xyz,
                    size: [
                        Dimension { length: size[0], axis: AxisSize::Width },
                        Dimension { length: size[1], axis: AxisSize::Height },
                        Dimension { length: size[2], axis: AxisSize::Depth },
                    ],
                });
                break;
            } 
            else if eq_tol(b_dim.length, item_dims[2].length) {
                side_1 = Some(i);
                break;
            }
        }

        // If no suitable side was found, just go for the first fit
        if side_1.is_none() {
            for (i, b_dim) in b_dims.iter().enumerate() {
                if b_dim.length >= item_dims[2].length {
                    side_1 = Some(i);

                    // Create remainder block
                    let mut xyz = space.position_xyz.clone();
                    let mut size = [item.size[0].length, item.size[1].length, item.size[2].length];
                    xyz[b_dim.axis] += item_dims[2].length;
                    size[b_dim.axis] = b_dim.length - item_dims[2].length;

                    remainder_blocks.push(Space {
                        position_xyz: xyz,
                        size: [
                            Dimension { length: size[0], axis: AxisSize::Width },
                            Dimension { length: size[1], axis: AxisSize::Height },
                            Dimension { length: size[2], axis: AxisSize::Depth },
                        ],
                    });
                    break;
                }
            }
        }

        let mut dim_1 = item_dims[0].clone();
        let mut dim_2 = item_dims[1].clone();
        let mut dim_3 = item_dims[2].clone();
        dim_3.axis = b_dims[side_1.unwrap()].axis.clone(); // Orient the longest item side to the chosen box side

        // Determine the orientation for the other two sides, preferring the combination that leaves the largest contiguous leftover volume
        let (side_2, side_3) = Self::get_side_2_side_3(&item_dims, &b_dims, side_1.unwrap());
        dim_2.axis = b_dims[side_2].axis.clone();
        dim_1.axis = b_dims[side_3].axis.clone();

        // Calculate how to split up the remaining space after occupation, of which there are two options:
        let block_2a: Space;
        let block_3a: Space;
        let block_2b: Space;
        let block_3b: Space;
        {
            let mut xyz = space.position_xyz.clone();
            let mut size = space_xyz_dims.clone();
            xyz[b_dims[side_3].axis] += item_dims[0].length;
            size[b_dims[side_3].axis] -= item_dims[0].length;
            block_2a = Space {
                position_xyz: xyz,
                size: [
                    Dimension { length: size[0], axis: AxisSize::Width },
                    Dimension { length: size[1], axis: AxisSize::Height },
                    Dimension { length: size[2], axis: AxisSize::Depth },
                ],
            };
        }
        {
            let mut xyz = space.position_xyz.clone();
            let mut size = space_xyz_dims.clone();
            xyz[b_dims[side_2].axis] += item_dims[1].length;
            size[b_dims[side_2].axis] -= item_dims[1].length;

            size[b_dims[side_3].axis] = item_dims[0].length;
            block_3a = Space {
                position_xyz: xyz,
                size: [
                    Dimension { length: size[0], axis: AxisSize::Width },
                    Dimension { length: size[1], axis: AxisSize::Height },
                    Dimension { length: size[2], axis: AxisSize::Depth },
                ],
            };
        }
        {
            let mut xyz = space.position_xyz.clone();
            let mut size = space_xyz_dims.clone();
            xyz[b_dims[side_2].axis] += item_dims[1].length;
            size[b_dims[side_2].axis] = b_dims[side_2].length - item_dims[1].length;
            block_2b = Space {
                position_xyz: xyz,
                size: [
                    Dimension { length: size[0], axis: AxisSize::Width },
                    Dimension { length: size[1], axis: AxisSize::Height },
                    Dimension { length: size[2], axis: AxisSize::Depth },
                ],
            };
        }
        {
            let mut xyz = space.position_xyz.clone();
            let mut size = space_xyz_dims.clone();
            xyz[b_dims[side_3].axis] += item_dims[0].length;
            size[b_dims[side_3].axis] = b_dims[side_3].length - item_dims[0].length;

            size[b_dims[side_2].axis] = item_dims[1].length;
            block_3b = Space {
                position_xyz: xyz,
                size: [
                    Dimension { length: size[0], axis: AxisSize::Width },
                    Dimension { length: size[1], axis: AxisSize::Height },
                    Dimension { length: size[2], axis: AxisSize::Depth },
                ],
            };
        }

        // Choose the combination that has the largest contiguous volume
        if block_2a.volume() > block_2b.volume() {
            remainder_blocks.push(block_2a);
            remainder_blocks.push(block_3a);
        } else {
            remainder_blocks.push(block_2b);
            remainder_blocks.push(block_3b);
        }

        // Remove any space with 0 volume
        let filtered: Vec<Space> = remainder_blocks
            .into_iter()
            .filter(|s| !eq_tol(s.volume(), 0.0))
            .collect();

        ([dim_1, dim_2, dim_3], filtered )
    }

    // Determines the rotation method by checking if the item MUST be rotated in a specific direction
    // based on size constraints, then returns the sides that leave the largest bulk volume in the box.
    // item_dims and box_dims are assumed to be sorted in ascending size.
    fn get_side_2_side_3(item_dims: &[Dimension], box_dims: &[Dimension], side_1: usize) -> (usize, usize) {
        let side_2: usize;
        let side_3: usize;

        let minus_1 = (side_1 + 2) % 3; 
        let minus_2 = (side_1 + 1) % 3;

        if item_dims[1].length > box_dims[minus_1].length {
            side_2 = minus_2;
            side_3 = minus_1;
        } else if item_dims[1].length > box_dims[minus_2].length {
            side_2 = minus_1;
            side_3 = minus_2;
        } else {
            side_2 = (side_1 + 1) % 3;
            side_3 = (side_1 + 2) % 3;
        }

        (side_2, side_3)
    }

}

// Compare f64 with an acceptable tolerance for packing purposes.
//  Currently an arbitrary value. The value doesn't need to be very precise, 
//  because package measurements probably aren't that precise anyways.
fn eq_tol(a: f64, b:f64) -> bool {
    const TOLERANCE: f64 = 0.0000001;
    (a - b).abs() <= TOLERANCE
}

// Check that an Item can fit into a Space, based on their dimensions
fn fits(container: &Space, to_fit: &Item) -> bool {
    let mut sorted_size_a = container.as_xyz();
    sorted_size_a.sort_by( |a, b|{
        b.partial_cmp(&a).unwrap() }); 

    let mut sorted_size_b = to_fit.as_xyz();
    sorted_size_b.sort_by( |a, b|{
        b.partial_cmp(&a).unwrap() }); 

    sorted_size_a[0] >= sorted_size_b[0] && 
    sorted_size_a[1] >= sorted_size_b[1] && 
    sorted_size_a[2] >= sorted_size_b[2]
}


// ----------------------------------------------------------------
// Unit tests
// TODO: Flesh out tests more! Ideally, these should be run during development, but the logic layout changed so much throughout
//  that doing them after the fact was easier...
//  - Check for overlapping items (expensive with many items, but no way around it)
//  - Isolate orientation case logic

#[test]
fn test_pack_all_items_in_bin() {
    // Test data: 10x10x10 bin, three items that fit
    let bin = Bin {
        width: 10.0,
        height: 10.0,
        depth: 10.0,
    };
    let items: Vec<Item> = vec![
        Item {
            id: 0,
            name: "item_1".to_string(),
            position_xyz: [0.0, 0.0, 0.0],
            size: [
                Dimension { length: 5.0, axis: AxisSize::Width },
                Dimension { length: 5.0, axis: AxisSize::Height },
                Dimension { length: 5.0, axis: AxisSize::Depth },
            ],
        },
        Item {
            id: 1,
            name: "item_2".to_string(),
            position_xyz: [0.0, 0.0, 0.0],
            size: [
                Dimension { length: 10.0, axis: AxisSize::Width },
                Dimension { length: 5.0, axis: AxisSize::Height },
                Dimension { length: 10.0, axis: AxisSize::Depth },
            ],
        },
        Item {
            id: 2,
            name: "item_3".to_string(),
            position_xyz: [0.0, 0.0, 0.0],
            size: [
                Dimension { length: 10.0, axis: AxisSize::Width },
                Dimension { length: 5.0, axis: AxisSize::Height },
                Dimension { length: 5.0, axis: AxisSize::Depth },
            ],
        },
    ];

    // Pack the items into the bin
    let result = BinPacker3D::pack(bin, items);

    // Assert that all items were placed
    assert_eq!(result.placed.len(), 3, "Both items should be placed in the bin");
    assert_eq!(result.unplaced.len(), 0, "No items should be unplaced");

    // Verify that each placed item is within the bin bounds
    for item in &result.placed {
        assert!(
            item.position_xyz[0] + item.size[0].length <= result.bin.width,
            "Item {} extends beyond bin width",
            item.id
        );
        assert!(
            item.position_xyz[1] + item.size[1].length <= result.bin.height,
            "Item {} extends beyond bin height",
            item.id
        );
        assert!(
            item.position_xyz[2] + item.size[2].length <= result.bin.depth,
            "Item {} extends beyond bin depth",
            item.id
        );
    }
}