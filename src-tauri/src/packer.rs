use std::collections::HashSet;
use std::time::Instant;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Bin {
    pub width: f64,
    pub height: f64,
    pub depth: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    // TODO: Add units of measurement, configurable per item
    pub width: f64,
    pub height: f64,
    pub depth: f64,
}
impl Item {
    fn volume(&self) -> f64 {
        self.width * self.height * self.depth
    }

    fn size(&self) -> [f64; 3] {
        [self.width, self.height, self.depth]
    }
}

struct Block {
    pub position: Vec<Dimension>,
    pub size: Vec<Dimension>
}
#[derive(Clone, Debug)]
struct Dimension {
    pub length: f64,
    pub axis: AxisSize
}
#[derive(Clone, Debug, PartialEq)]
enum AxisSize {
    Width,
    Height,
    Depth
}

#[derive(Clone, Debug, PartialEq)]
struct Orientation {
    width: f64,
    height: f64,
    depth: f64,
}

#[derive(Clone, Debug)]
struct Space {
    x: f64,
    y: f64,
    z: f64,
    width: f64,
    height: f64,
    depth: f64,
}
impl Space {
    fn volume(&self) -> f64 {
        self.width * self.height * self.depth
    }
    fn size(&self) -> [f64; 3] {
        [self.width, self.height, self.depth]
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

        // Sort items by largest dimension, descending - This will be the order we process items in.
        let mut sorted_items = items;

        // Move any items that are larger than the bin dimensions to the unplaced list first, since they can never be placed.
        // Although they might fit diagonally for a more optimal solution, for simplicity we only try 90 degree rotations.
        sorted_items.retain(|item| {
            let item_max = item.width.max(item.height).max(item.depth);
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
            let max_dim_a = a.width.max(a.height).max(a.depth);
            let max_dim_b = b.width.max(b.height).max(b.depth);
            max_dim_b.partial_cmp(&max_dim_a).unwrap()
        });

        // Define all the largest possible blocks of free 'Space' in the bin.
        // Each block is a candidate for placing an item into, and may overlap over each other.
        // Must be updated when bin items change, since the possible placements will change.
        let mut free_spaces = vec![Space {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            width: bin.width,
            height: bin.height,
            depth: bin.depth,
        }];

        for item in sorted_items {
            let mut best_fit: Option<(usize, Orientation, Space, Vec<Space>)> = None;

            // Find a space to fit the item
            for (index, space) in free_spaces.iter().enumerate() {

                if fits(space, &item) {
                    let (orientation, remainder) = Self::best_orientation(space, &item);

                    best_fit = Some((index, orientation, space.clone(), remainder));
                    break;
                }
            }

            if let Some((space_index, orientation, space, remainder)) = best_fit {
                // Space found, place the item
                let mut placed_item = item.clone();
                placed_item.x = space.x;
                placed_item.y = space.y;
                placed_item.z = space.z;
                placed_item.width = orientation.width;
                placed_item.height = orientation.height;
                placed_item.depth = orientation.depth;

                // Place the object and consume the space
                placed.push(placed_item.clone());
                free_spaces.remove(space_index);

                // Update space blocks
                // let new_spaces = split_space(&space, &placed_item, &free_spaces);
                // free_spaces = Self::clean_spaces([free_spaces, new_spaces].concat());
                free_spaces = Self::clean_spaces([free_spaces, remainder].concat());

            } else { 
                // No possible spaces found! Try the next item.
                unplaced.push(item);
            }
        }

        let time_taken = start_time.elapsed().as_millis();

        // Calculate bin usage percentage
        let bin_volume = bin.width * bin.height * bin.depth;
        let used_volume: f64 = placed.iter().map(|item| item.volume()).sum();
        let bin_usage_percentage = (used_volume / bin_volume) * 100.0;

        PackResult {
            bin,
            placed,
            unplaced,
            time_to_pack: time_taken,
            bin_usage_percentage,
        }
    }

    fn best_orientation(space: &Space, item: &Item) -> (Orientation, Vec<Space>) {

        fn item_to_dimensions(item: &Item) -> Vec<Dimension> {
            vec![
                Dimension {
                    length: item.width,
                    axis: AxisSize::Width,
                },
                Dimension {
                    length: item.height,
                    axis: AxisSize::Height,
                },
                Dimension {
                    length: item.depth,
                    axis: AxisSize::Depth,
                },
            ]
        }
        fn space_to_dimensions(space: &Space) -> Vec<Dimension> {
            vec![
                Dimension {
                    length: space.width,
                    axis: AxisSize::Width,
                },
                Dimension {
                    length: space.height,
                    axis: AxisSize::Height,
                },
                Dimension {
                    length: space.depth,
                    axis: AxisSize::Depth,
                },
            ]
        }

        let mut space_dims = [space.width, space.height, space.depth];
        let mut container: Vec<Dimension> = space_to_dimensions(&space);
        let mut to_pack: Vec<Dimension> = item_to_dimensions(item);
        let mut remainder_blocks: Vec<Space> = Vec::new();

        // Sort dimensions ascending (shortest first)
        container.sort_by(|a, b| a.length.partial_cmp(&b.length).unwrap());
        to_pack.sort_by(|a, b| a.length.partial_cmp(&b.length).unwrap());

        let longest_item_dim = to_pack[2].length;
        let mut side_1: Option<usize> = None;

        // Choose the shortest side of the box we can stack the item twice on its longest side
        for (i, b_dim) in container.iter().enumerate() {
            if b_dim.length >= longest_item_dim * 2.0 {
                side_1 = Some(i);

                // Create remainder block along fitted axis
                let mut xyz = [space.x, space.y, space.z];
                let mut size = [item.width, item.height, item.depth];
                let extension_index = match container[i].axis {
                    AxisSize::Width => 0,
                    AxisSize::Height => 1,
                    AxisSize::Depth => 2,
                };

                println!("Extending on axis: {:?} by {}", container[i].axis, to_pack[2].length);

                xyz[extension_index] += to_pack[2].length;
                size[extension_index] = space_dims[extension_index] - to_pack[2].length;

                remainder_blocks.push(Space {
                    x: xyz[0],
                    y: xyz[1],
                    z: xyz[2],
                    width: size[0],
                    height: size[1],
                    depth: size[2],
                });
                println!("2x fit on axis {:?}, remainder block: {}x{}x{} at {},{},{}", b_dim.axis ,size[0], size[1], size[2], xyz[0], xyz[1], xyz[2]);

                // space_dims[extension_index] = to_pack[2].length;
                // container[i].length = to_pack[2].length;
                break;
            } 
            // Otherwise, try for an exact fit
            else if eq_tol(b_dim.length, to_pack[2].length) {
                side_1 = Some(i);

                // No remainder block due to exact fit
                println!("exact fit, no remainder block");
                break;
            }
        }
        // If no suitable side was found, just fit the item
        if side_1.is_none() {
            for (i, b_dim) in container.iter().enumerate() {
                if b_dim.length >= longest_item_dim {
                    side_1 = Some(i);

                    // Create remainder block
                    let mut xyz = [space.x, space.y, space.z];
                    let mut size = [item.width, item.height, item.depth];
                    let extension_index = match container[i].axis {
                        AxisSize::Width => 0,
                        AxisSize::Height => 1,
                        AxisSize::Depth => 2,
                    };
                    xyz[extension_index] += longest_item_dim;
                    size[extension_index] = container[i].length - longest_item_dim;

                    remainder_blocks.push(Space {
                        x: xyz[0],
                        y: xyz[1],
                        z: xyz[2],
                        width: size[0],
                        height: size[1],
                        depth: size[2],
                    });

                    println!("1x fit, remainder block: {}x{}x{} at {},{},{}", size[0], size[1], size[2], xyz[0], xyz[1], xyz[2]);
                    break;
                }
            }
        }

        let mut dim_1 = to_pack[0].clone();
        let mut dim_2 = to_pack[1].clone();
        let mut dim_3 = to_pack[2].clone();
        dim_3.axis = container[side_1.unwrap()].axis.clone();

        let (side_2, side_3) = Self::get_side_2_side_3(&to_pack, &container, side_1.unwrap());
        dim_2.axis = container[side_2].axis.clone();
        dim_1.axis = container[side_3].axis.clone();

        // Create the remaining two blocks of space
        let block_2a: Space;
        let block_3a: Space;
        let block_2b: Space;
        let block_3b: Space;

        {
            let mut xyz = [space.x, space.y, space.z];
            let mut size = space_dims.clone();
            let extension_index = match container[side_3].axis {
                AxisSize::Width => 0,
                AxisSize::Height => 1,
                AxisSize::Depth => 2,
            };
            xyz[extension_index] += to_pack[0].length;
            size[extension_index] -= to_pack[0].length;
            block_2a = Space {
                x: xyz[0],
                y: xyz[1],
                z: xyz[2],
                width: size[0],
                height: size[1],
                depth: size[2],
            };
        }
        {
            let mut xyz = [space.x, space.y, space.z];
            let mut size = space_dims.clone();
            let extension_index = match container[side_2].axis {
                AxisSize::Width => 0,
                AxisSize::Height => 1,
                AxisSize::Depth => 2,
            };
            xyz[extension_index] += to_pack[1].length;
            size[extension_index] -= to_pack[1].length;
            let smallest_index = match container[side_3].axis {
                AxisSize::Width => 0,
                AxisSize::Height => 1,
                AxisSize::Depth => 2,
            };
            size[smallest_index] = to_pack[0].length;
            block_3a = Space {
                x: xyz[0],
                y: xyz[1],
                z: xyz[2],
                width: size[0],
                height: size[1],
                depth: size[2],
            };
        }
        {
            let mut xyz = [space.x, space.y, space.z];
            let mut size = space_dims.clone();
            let extension_index = match container[side_2].axis {
                AxisSize::Width => 0,
                AxisSize::Height => 1,
                AxisSize::Depth => 2,
            };
            xyz[extension_index] += to_pack[1].length;
            size[extension_index] = container[side_2].length - to_pack[1].length;
            block_2b = Space {
                x: xyz[0],
                y: xyz[1],
                z: xyz[2],
                width: size[0],
                height: size[1],
                depth: size[2],
            };
        }
        {
            let mut xyz = [space.x, space.y, space.z];
            let mut size = space_dims.clone();
            let extension_index = match container[side_3].axis {
                AxisSize::Width => 0,
                AxisSize::Height => 1,
                AxisSize::Depth => 2,
            };
            xyz[extension_index] += to_pack[0].length;
            size[extension_index] = container[side_3].length - to_pack[0].length;
            let smallest_index = match container[side_2].axis {
                AxisSize::Width => 0,
                AxisSize::Height => 1,
                AxisSize::Depth => 2,
            };
            size[smallest_index] = to_pack[1].length;
            block_3b = Space {
                x: xyz[0],
                y: xyz[1],
                z: xyz[2],
                width: size[0],
                height: size[1],
                depth: size[2],
            };
        }

        if block_2a.volume() > block_2b.volume() {
            println!("2a 3a, remainder blocks: {}x{}x{} at {},{},{} and {}x{}x{} at {},{},{}", 
                block_2a.width, block_2a.height, block_2a.depth, block_2a.x, block_2a.y, block_2a.z,
                block_3a.width, block_3a.height, block_3a.depth, block_3a.x, block_3a.y, block_3a.z
            );
            remainder_blocks.push(block_2a);
            remainder_blocks.push(block_3a);
        } else {
            println!("2b 3b, remainder blocks: {}x{}x{} at {},{},{} and {}x{}x{} at {},{},{}", 
                block_2b.width, block_2b.height, block_2b.depth, block_2b.x, block_2b.y, block_2b.z,
                block_3b.width, block_3b.height, block_3b.depth, block_3b.x, block_3b.y, block_3b.z
            );
            remainder_blocks.push(block_2b);
            remainder_blocks.push(block_3b);
        }
        
        // Arrange dims back into width/height/depth
        let unarranged = vec![dim_1, dim_2, dim_3];
        let mut orientation = vec![0.0; 3];
        for dim in unarranged.iter() {
            match dim.axis {
                AxisSize::Width => orientation[0] = dim.length,
                AxisSize::Height => orientation[1] = dim.length,
                AxisSize::Depth => orientation[2] = dim.length,
            }
        }

        // Remove any space with 0 volume
        let filtered: Vec<Space> = remainder_blocks
            .into_iter()
            .filter(|s| !eq_tol(s.volume(), 0.0))
            .collect();

        println!("Remainder blocks:");
        for block in filtered.iter() {
            println!("{}x{}x{} at {},{},{}", block.width, block.height, block.depth, block.x, block.y, block.z);
        }

        ( Orientation {
            width: orientation[0],
            height: orientation[1],
            depth: orientation[2],
        }, 
        filtered )
    }

    // Determines the rotation method by checking if the item MUST be rotated in a specific direction
    // based on size constraints, then returns the sides that leave the largest bulk volume in the box.
    //
    // Args:
    //   item_dims: Vec<Dimension> representing item dimensions (width, height, depth)
    //   box_dims: Vec<Dimension> representing box/container dimensions
    //   side_1: usize - index of the side of the box the item is placed along
    //
    // Returns: (usize, usize) - indexes of the box sides the items will be placed along
    fn get_side_2_side_3(item_dims: &[Dimension], box_dims: &[Dimension], side_1: usize) -> (usize, usize) {
        let side_2: usize;
        let side_3: usize;

        fn wraparound(index: i8) -> usize {
            match index {
                -1 => 2,
                -2 => 1,
                i => i as usize,
            }
        }
        let minus_1 = wraparound(side_1 as i8 - 1); // side_1 - 1 with wraparound
        let minus_2 = wraparound(side_1 as i8 - 2); // side_1 - 2 with wraparound

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

    // Removes invalid spaces (with zero or negative dimensions) and deduplicates identical spaces
    // Returns a cleaned vector of valid, unique free spaces
    fn clean_spaces(spaces: Vec<Space>) -> Vec<Space> {

        // Remove any space with 0 volume
        let filtered: Vec<Space> = spaces
            .into_iter()
            .filter(|s| !eq_tol(s.volume(), 0.0))
            .collect();

        // Deduplicate identical spaces
        let mut deduplicated: Vec<Space> = Vec::new();
        let mut unique_spaces = HashSet::new();
        for space in filtered {
            let key = format!(
                "{}.{}.{},{}x{}x{}", 
                space.x, space.y, space.z,
                space.width, space.height, space.depth
            );
            if !unique_spaces.contains(&key) {
                unique_spaces.insert(key);
                deduplicated.push(space);
            }
        }

        // Arrange by smallest space first
        deduplicated.sort_by(|a, b| {
            a.volume().partial_cmp(&b.volume()).unwrap()
        });

        deduplicated
    }

}

// Compare f64 with an acceptable tolerance for packing purposes.
// TODO: Justify tolerance value. Currently an arbitrary value, but the idea is to use metric cm as the standard.
//  The value doesn't need to be very precise, because package measurements probably aren't that precise anyways.
fn eq_tol(a: f64, b:f64) -> bool {
    const TOLERANCE: f64 = 0.001;
    (a - b).abs() <= TOLERANCE
}

// Check that an Item can fit into a Space, based on their dimensions
fn fits(container: &Space, to_fit: &Item) -> bool {
    let mut sorted_size_a = container.size();
    sorted_size_a.sort_by( |a, b|{
        b.partial_cmp(&a).unwrap() }); 

    let mut sorted_size_b = to_fit.size();
    sorted_size_b.sort_by( |a, b|{
        b.partial_cmp(&a).unwrap() }); 

    sorted_size_a[0] >= sorted_size_b[0] && 
    sorted_size_a[1] >= sorted_size_b[1] && 
    sorted_size_a[2] >= sorted_size_b[2]
}


// Split a block of 'Space' up to three times, creating smaller blocks of leftover space surrounding an occupying item.
// Note that items are inserted at the bottom left corner, which is why we only ever need to define three new blocks 
// to the right, top, and front of the occupying item.
// Returns a vector of Space objects.
fn split_space(space: &Space, occupied: &Item, spaces: &Vec<Space>) -> Vec<Space> {
    #[derive(Clone)]
    enum Direction {
        Right,
        Top,
        Front,
    }
    struct Subspace {
        space: Space,
        dir: Direction,
    }
    let mut subspaces: Vec<Subspace> = Vec::new();

    // Max adjacent blocks
    let max_right = Space {
        x: occupied.x + occupied.width,
        y: occupied.y,
        z: occupied.z,
        width: space.width - occupied.width,
        height: space.height,
        depth: space.depth,
    };
    let max_top = Space {
        x: occupied.x,
        y: occupied.y + occupied.height,
        z: occupied.z,
        width: space.width,
        height: space.height - occupied.height,
        depth: space.depth,
    };
    let max_front: Space = Space {
        x: occupied.x,
        y: occupied.y,
        z: occupied.z + occupied.depth,
        width: space.width,
        height: space.height,
        depth: space.depth - occupied.depth,
    };

    // WIP: Reduce overlapping spaces
    // Check if the new space is actually part of a larger contiguous block
    // let is_subset = |candidate: &Space| {
    //     for space in spaces { 
    //         // Case 1: Shared width and height (differ in depth/z-axis)
    //         if eq_tol(space.width, candidate.width) && eq_tol(space.height, candidate.height) 
    //             && eq_tol(space.x, candidate.x) && eq_tol(space.y, candidate.y) 
    //             // && candidate.z <= space.z + space.depth || candidate.z >= space.z - space.depth
    //         {
    //             return true;
    //         }
            
    //         // Case 2: Shared width and depth (differ in height/y-axis)
    //         if eq_tol(space.width, candidate.width) && eq_tol(space.depth, candidate.depth) 
    //             && eq_tol(space.x, candidate.x) && eq_tol(space.z, candidate.z)
    //             // && candidate.y <= space.y + space.height || candidate.y >= space.y - candidate.height
    //         {
    //             return true;
    //         }
            
    //         // Case 3: Shared height and depth (differ in width/x-axis)
    //         if eq_tol(space.height, candidate.height) && eq_tol(space.depth, candidate.depth) 
    //             && eq_tol(space.y, candidate.y) && eq_tol(space.z, candidate.z)
    //             // && candidate.x <= space.x + space.width || candidate.x >= space.x - candidate.width
    //         {
    //             return true;
    //         }
    //     }
    //     false
    // };
    // if max_right.volume() > 0.0 {
    //     if !is_subset(&max_right) {
    //         subspaces.push(Subspace { space: max_right, dir: Direction::Right });
    //     }
    // }
    // if max_top.volume() > 0.0 {
    //     if !is_subset(&max_top) {
    //         subspaces.push(Subspace { space: max_top, dir: Direction::Top });
    //     }
    // }
    // if max_front.volume() > 0.0 {
    //     if !is_subset(&max_front) {
    //         subspaces.push(Subspace { space: max_front, dir: Direction::Front });
    //     }
    // }
    // if subspaces.is_empty() { // No leftover space, return an empty vector
    //     return Vec::new();
    // }


    // TEST: No overlap blocks, todo: optimise decision logic between front/right blocks
    let max_right = Space {
        x: occupied.x + occupied.width,
        y: occupied.y,
        z: occupied.z,
        width: space.width - occupied.width,
        height: space.height,
        depth: occupied.depth,
    };
    let max_top = Space {
        x: occupied.x,
        y: occupied.y + occupied.height,
        z: occupied.z,
        width: occupied.width,
        height: space.height - occupied.height,
        depth: occupied.depth,
    };
    let max_front: Space = Space {
        x: occupied.x,
        y: occupied.y,
        z: occupied.z + occupied.depth,
        width: space.width,
        height: space.height,
        depth: space.depth - occupied.depth,
    };

    if max_right.volume() > 0.0 {
            subspaces.push(Subspace { space: max_right, dir: Direction::Right });
    }
    if max_top.volume() > 0.0 {
            subspaces.push(Subspace { space: max_top, dir: Direction::Top });
    }
    if max_front.volume() > 0.0 {
            subspaces.push(Subspace { space: max_front, dir: Direction::Front });
    }
    if subspaces.is_empty() { // No leftover space, return an empty vector
        return Vec::new();
    }

    subspaces.into_iter().map(|s| s.space).collect()
}

// ----------------------------------------------------------------
// Unit tests

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
            x: 0.0,
            y: 0.0,
            z: 0.0,
            width: 5.0,
            height: 5.0,
            depth: 5.0,
        },
        Item {
            id: 1,
            name: "item_2".to_string(),
            x: 0.0,
            y: 0.0,
            z: 0.0,
            width: 10.0,
            height: 5.0,
            depth: 10.0,
        },
        Item {
            id: 2,
            name: "item_3".to_string(),
            x: 0.0,
            y: 0.0,
            z: 0.0,
            width: 10.0,
            height: 5.0,
            depth: 5.0,
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
            item.x + item.width <= result.bin.width,
            "Item {} extends beyond bin width",
            item.id
        );
        assert!(
            item.y + item.height <= result.bin.height,
            "Item {} extends beyond bin height",
            item.id
        );
        assert!(
            item.z + item.depth <= result.bin.depth,
            "Item {} extends beyond bin depth",
            item.id
        );
    }
}

#[test]
fn test_pack_1000_items() {
    let bin = Bin {
        width: 30.0,
        height: 30.0,
        depth: 30.0,
    };

    // Create three types of items with different dimensions
    // Type 1: Small items (2x2x2)
    // Type 2: Medium items (3x3x3)
    // Type 3: Large items (4x4x4)

    let mut items: Vec<Item> = Vec::new();
    let mut item_id = 0;

    // Add 333 small items
    for _ in 0..333 {
        items.push(Item {
            id: item_id,
            name: "small".to_string(),
            x: 0.0,
            y: 0.0,
            z: 0.0,
            width: 2.0,
            height: 2.0,
            depth: 2.0,
        });
        item_id += 1;
    }

    // Add 334 medium items
    for _ in 0..334 {
        items.push(Item {
            id: item_id,
            name: "medium".to_string(),
            x: 0.0,
            y: 0.0,
            z: 0.0,
            width: 3.0,
            height: 3.0,
            depth: 3.0,
        });
        item_id += 1;
    }

    // Add 333 large items
    for _ in 0..333 {
        items.push(Item {
            id: item_id,
            name: "large".to_string(),
            x: 0.0,
            y: 0.0,
            z: 0.0,
            width: 4.0,
            height: 4.0,
            depth: 4.0,
        });
        item_id += 1;
    }

    assert_eq!(items.len(), 1000, "Should have created 1000 items total");

    // Pack the items into the bin
    let result = BinPacker3D::pack(bin, items);

    // Print the packing time for performance analysis
    println!("Container: {}x{}x{}", result.bin.width, result.bin.height, result.bin.depth);
    println!("Packed {} items, {} items could not be packed", result.placed.len(), result.unplaced.len());
    println!("Time taken to pack: {} ms", result.time_to_pack);
    println!("Bin usage percentage: {:.2}%", result.bin_usage_percentage);

    // // Verify that all items were placed
    // assert!(
    //     result.unplaced.len() == 0,
    //     "All items should be placed in the bin, but {} were unplaced",
    //     result.unplaced.len()
    // );

    // // Verify that each placed item is within the bin bounds
    // for item in &result.placed {
    //     assert!(
    //         item.x >= 0.0 && item.x + item.width <= result.bin.width,
    //         "Item {} x position out of bounds",
    //         item.id
    //     );
    //     assert!(
    //         item.y >= 0.0 && item.y + item.height <= result.bin.height,
    //         "Item {} y position out of bounds",
    //         item.id
    //     );
    //     assert!(
    //         item.z >= 0.0 && item.z + item.depth <= result.bin.depth,
    //         "Item {} z position out of bounds",
    //         item.id
    //     );
    // }

    // // Verify no items overlap
    // for (i, item1) in result.placed.iter().enumerate() {
    //     for item2 in result.placed.iter().skip(i + 1) {
    //         let no_overlap_x = item1.x + item1.width <= item2.x || item2.x + item2.width <= item1.x;
    //         let no_overlap_y =
    //             item1.y + item1.height <= item2.y || item2.y + item2.height <= item1.y;
    //         let no_overlap_z =
    //             item1.z + item1.depth <= item2.z || item2.z + item2.depth <= item1.z;

    //         assert!(
    //             no_overlap_x || no_overlap_y || no_overlap_z,
    //             "Items {} and {} overlap",
    //             item1.id,
    //             item2.id
    //         );
    //     }
    // }
}