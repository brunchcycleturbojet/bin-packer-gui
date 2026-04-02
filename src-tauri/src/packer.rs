use std::collections::HashSet;
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
    pub width: f64,
    pub height: f64,
    pub depth: f64,
    pub rotate_x: f64,
    pub rotate_y: f64,
    pub rotate_z: f64,
}

#[derive(Clone, Debug, PartialEq)]
struct Orientation {
    width: f64,
    height: f64,
    depth: f64,
    rotate_x: f64,
    rotate_y: f64,
    rotate_z: f64,
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
}

pub struct PackResult {
    pub bin: Bin,
    pub placed: Vec<Item>,
    pub unplaced: Vec<Item>,
    // TODO: Add time taken
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
        sorted_items.sort_by(|a, b| { // Largest dimension primary
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
            let mut least_leftover: Option<(usize, Orientation, f64)> = None;
            let mut best_space: Option<Space> = None;

            // Rotate the item in all possible orientations, and place it in the space that leaves the largest leftover volume
            for (si, space) in free_spaces.iter().enumerate() {
                for ori in Self::orientations(&item) {
                    // Check if item fits in space without intersecting with any other already packed items
                    if ori.width <= space.width && ori.height <= space.height && ori.depth <= space.depth
                        && !Self::intersects_with_any(space.x, space.y, space.z, ori.width, ori.height, ori.depth, &placed) {

                        let leftover = space.width * space.height * space.depth - ori.width * ori.height * ori.depth;
                        if least_leftover.is_none() || leftover < least_leftover.as_ref().unwrap().2 {
                            least_leftover = Some((si, ori, leftover));
                            best_space = Some(space.clone());
                        }
                    }
                    // else: Move onto the next orientation, because it doesn't fit this way.
                }
            }

            // Found the smallest space to fit the item into, place it and then update the available space blocks.
            if let (Some((space_index, orientation, _)), Some(space)) = (least_leftover, best_space) {
                let mut placed_item = item.clone();
                placed_item.x = space.x;
                placed_item.y = space.y;
                placed_item.z = space.z;
                placed_item.width = orientation.width;
                placed_item.height = orientation.height;
                placed_item.depth = orientation.depth;
                placed_item.rotate_x = orientation.rotate_x;
                placed_item.rotate_y = orientation.rotate_y;
                placed_item.rotate_z = orientation.rotate_z;

                placed.push(placed_item.clone());

                free_spaces.remove(space_index);

                let new_spaces = Self::split_space(&space, &placed_item);
                free_spaces = Self::clean_spaces([free_spaces, new_spaces].concat());

            } else { // No possible spaces found! Try the next item, which should be smaller than this one, since we process by descending volume.
                unplaced.push(item);
            }
        }

        PackResult {
            bin,
            placed,
            unplaced,
        }
    }

    // Checks if a box with given position and dimensions intersects with any item in a list
    fn intersects_with_any(x: f64, y: f64, z: f64, width: f64, height: f64, depth: f64, items: &[Item]) -> bool {
        items.iter().any(|other| Self::intersects(x, y, z, width, height, depth, other.x, other.y, other.z, other.width, other.height, other.depth))
    }

    // Checks if two 3D axis-aligned boxes intersect
    // Returns true if they overlap, false if they don't
    fn intersects(x1: f64, y1: f64, z1: f64, w1: f64, h1: f64, d1: f64, x2: f64, y2: f64, z2: f64, w2: f64, h2: f64, d2: f64) -> bool {
        // Two boxes don't intersect if one is completely to the side of the other on any axis
        // They intersect if none of these conditions are true:
        let no_overlap_x = x1 + w1 <= x2 || x2 + w2 <= x1;
        let no_overlap_y = y1 + h1 <= y2 || y2 + h2 <= y1;
        let no_overlap_z = z1 + d1 <= z2 || z2 + d2 <= z1;

        // If there's overlap on all three axes, the boxes intersect
        !no_overlap_x && !no_overlap_y && !no_overlap_z
    }

    // Generates all unique axis-aligned orientations for an item by rotating it in 3D space
    // Returns up to 6 different orientations (some may be identical if item is symmetric)
    fn orientations(item: &Item) -> Vec<Orientation> {
        let dims = [item.width, item.height, item.depth];
        let mut orientations = HashSet::new();
        let mut out = Vec::new();

        let triples = [
            (dims[0], dims[1], dims[2], 0.0, 0.0, 0.0),
            (dims[0], dims[2], dims[1], 0.0, 90.0, 0.0),
            (dims[1], dims[0], dims[2], 90.0, 0.0, 0.0),
            (dims[1], dims[2], dims[0], 0.0, 0.0, 90.0),
            (dims[2], dims[0], dims[1], 90.0, 0.0, 90.0),
            (dims[2], dims[1], dims[0], 0.0, 90.0, 90.0),
        ];

        for (w, h, d, rx, ry, rz) in triples.iter() {
            let key = format!("{}x{}x{}", w, h, d);
            if !orientations.contains(&key) {
                orientations.insert(key);
                out.push(Orientation {
                    width: *w,
                    height: *h,
                    depth: *d,
                    rotate_x: *rx,
                    rotate_y: *ry,
                    rotate_z: *rz,
                });
            }
        }

        out
    }

    // Split a block of 'Space' up to three times, creating smaller blocks of leftover space surrounding an occupying item.
    // Note that items are inserted at the bottom left corner, which is why we only ever need to define three new blocks 
    // to the right, top, and front of the occupying item.
    // Returns a vector of Space objects.
    fn split_space(space: &Space, occupied: &Item) -> Vec<Space> {
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

        if max_right.width > 0.0 && max_right.height > 0.0 && max_right.depth > 0.0 {
            subspaces.push(Subspace { space: max_right, dir: Direction::Right });
        }
        if max_top.width > 0.0 && max_top.height > 0.0 && max_top.depth > 0.0 {
            subspaces.push(Subspace { space: max_top, dir: Direction::Top });
        }
        if max_front.width > 0.0 && max_front.height > 0.0 && max_front.depth > 0.0 {
            subspaces.push(Subspace { space: max_front, dir: Direction::Front });
        }
        if subspaces.is_empty() { // No leftover space, return an empty vector
            return Vec::new();
        }

        // // Resolve any overlapping space
        // // Prefer the combination that has the largest singular volume of space at each step
        // subspaces.sort_by(|block_a, block_b| { 
        //     let a = &block_a.space;
        //     let b = &block_b.space;
        //     let max_dim_a = a.width.max(a.height).max(a.depth);
        //     let max_dim_b = b.width.max(b.height).max(b.depth);
        //     max_dim_b.partial_cmp(&max_dim_a).unwrap()
        // });

        // let subtract_block_dims = |smaller: &mut Subspace, bigger_dir: &Direction, bigger_space: &Space| {
        //     match bigger_dir {
        //         Direction::Right => smaller.space.width -= bigger_space.width,
        //         Direction::Front => smaller.space.depth -= bigger_space.depth,
        //         Direction::Top => smaller.space.height -= bigger_space.height
        //     }
        // };

        // if subspaces.get(1).is_some() { // Update dims to not overlap with the largest block
        //     let block_1_dir = subspaces[0].dir.clone();
        //     let block_1_space = subspaces[0].space.clone();
        //     subtract_block_dims(&mut subspaces[1], &block_1_dir, &block_1_space);
        // }
        // if subspaces.get(2).is_some() { // Update dims to not overlap with both the largest and middle block
        //     let block_1_dir = subspaces[0].dir.clone();
        //     let block_1_space = subspaces[0].space.clone();
        //     let block_2_dir = subspaces[1].dir.clone();
        //     let block_2_space = subspaces[1].space.clone();
        //     subtract_block_dims(&mut subspaces[2], &block_1_dir, &block_1_space);
        //     subtract_block_dims(&mut subspaces[2], &block_2_dir, &block_2_space);
        // }

        subspaces.into_iter().map(|s| s.space).collect()
    }

    // 
    // Removes invalid spaces (with zero or negative dimensions) and deduplicates identical spaces
    // Returns a cleaned vector of valid, unique free spaces
    fn clean_spaces(spaces: Vec<Space>) -> Vec<Space> {
        let filtered: Vec<Space> = spaces
            .into_iter()
            .filter(|s| s.width > 0.0 && s.height > 0.0 && s.depth > 0.0)
            .collect();

        let mut merged: Vec<Space> = Vec::new();
        for s in filtered {
            let mut merged_into_existing = false;
            for m in &merged {
                if s.x == m.x
                    && s.y == m.y
                    && s.z == m.z
                    && s.width == m.width
                    && s.height == m.height
                    && s.depth == m.depth
                {
                    merged_into_existing = true;
                    break;
                }
            }
            if !merged_into_existing {
                merged.push(s);
            }
        }
        merged
    }

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
            rotate_x: 0.0,
            rotate_y: 0.0,
            rotate_z: 0.0,
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
            rotate_x: 0.0,
            rotate_y: 0.0,
            rotate_z: 0.0,
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
            rotate_x: 0.0,
            rotate_y: 0.0,
            rotate_z: 0.0,
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