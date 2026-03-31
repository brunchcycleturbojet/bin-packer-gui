use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct Bin {
    pub id: String,
    pub width: f64,
    pub height: f64,
    pub depth: f64,
    pub items: Vec<Item>,
}

impl Bin {
    // Creates a new Bin with the given dimensions and an empty items list
    pub fn new(id: String, width: f64, height: f64, depth: f64) -> Self {
        Bin {
            id,
            width,
            height,
            depth,
            items: Vec::new(),
        }
    }
}

#[derive(Clone, Debug)]
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

pub struct PackResult {
    pub bin: Bin,
    pub placed: Vec<Item>,
    pub unplaced: Vec<Item>,
}

pub struct BinPacker3D;

// Heuristic 3D bin packing for one rectangular bin and items, with axis-aligned rotations only.
// An optimal solution is NOT guaranteed. The algorithms in use are approximations as the problem is NP-hard (as of writing!).
//
// Bin/Item origin is considered at the bottom left corner.
// Coordinates are such that X = width, Y = height (up), Z = depth.
impl BinPacker3D {

    // Packs items into one bin.
    // Returns a copy of the input bin, placed items with sorted position/rotations, and any unplaced items.
    pub fn pack(mut bin: Bin, items: Vec<Item>) -> PackResult {
        let mut unplaced = Vec::new();
        let mut placed = Vec::new();

        // Sort items by descending volume (largest first) - This will be the order we process items in.
        let mut sorted_items = items;
        sorted_items.sort_by(|a, b| {
            let vol_a = a.width * a.height * a.depth;
            let vol_b = b.width * b.height * b.depth;
            vol_b.partial_cmp(&vol_a).unwrap()
        });

        // Move any items that are larger than the bin dimensions to the unplaced list immediately, since they can never be placed.
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
            let mut smallest_waste: Option<(usize, Orientation, f64)> = None;
            let mut best_space: Option<Space> = None;

            // To find the smallest free space block we can place this item into,
            // try placing this item in each available block of free space,
            // across all possible orientations of the item.
            for (si, space) in free_spaces.iter().enumerate() {
                for ori in Self::orientations(&item) {
                    if (ori.width <= space.width && ori.height <= space.height && ori.depth <= space.depth) {

                        let waste = space.width * space.height * space.depth - ori.width * ori.height * ori.depth;
                        if smallest_waste.is_none() || waste < smallest_waste.as_ref().unwrap().2 {
                            smallest_waste = Some((si, ori, waste));
                            best_space = Some(space.clone());
                        }
                    }
                    // else: Move onto the next orientation, because it doesn't fit this way.
                }
            }

            // Found the smallest space to fit the item into, place it and then update the available space blocks.
            if let (Some((space_index, orientation, _)), Some(space)) = (smallest_waste, best_space) {
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
                bin.items.push(placed_item.clone());

                // Update the free spaces that intersect with the newly placed item

                


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

    //   TODO: Use collision detection to construct free spaces left/back as well
    // Split the volume of a free 'Space' block into smaller blocks, surrounding a newly added item.
    // Returns a vector of Space objects.
    fn split_space(space: &Space, reference_item: &Item) -> Vec<Space> {
        let mut out = Vec::new();

        let right = Space {
            x: reference_item.x + reference_item.width,
            y: reference_item.y,
            z: reference_item.z,
            width: space.width - reference_item.width,
            height: space.height,
            depth: space.depth,
        };

        let top = Space {
            x: reference_item.x,
            y: reference_item.y + reference_item.height,
            z: reference_item.z,
            width: space.width,
            height: space.height - reference_item.height,
            depth: space.depth,
        };

        let front: Space = Space {
            x: reference_item.x,
            y: reference_item.y,
            z: reference_item.z + reference_item.depth,
            width: space.width,
            height: space.height,
            depth: space.depth - reference_item.depth,
        };

        if right.width > 0.0 && right.height > 0.0 && right.depth > 0.0 {
            out.push(right);
        }
        if top.width > 0.0 && top.height > 0.0 && top.depth > 0.0 {
            out.push(top);
        }
        if front.width > 0.0 && front.height > 0.0 && front.depth > 0.0 {
            out.push(front);
        }

        out
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