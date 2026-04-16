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
    pub shape_id: i32, // Quantity is scrubbed when packing, so we keep track of a shape_id shared by identical items. 
    pub name: String,
    pub position_xyz: [f64; 3],
    pub size: [Dimension; 3], // Unordered width/height/depth
}
impl Dimensional for Item {
    fn get_size(&self) -> &[Dimension; 3] {
        &self.size
    }
}

#[derive(Clone, Debug)]
pub struct Space {
    pub position_xyz: [f64; 3],
    pub size: [Dimension; 3], // Unordered width/height/depth
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

    fn size_xyz(&self) -> [f64; 3] {
        let mut xyz = [0.0, 0.0, 0.0];
        for dim in self.get_size().iter() {
            xyz[dim.axis] = dim.length;
        }
        xyz
    }

    fn is_same_size(&self, other: &dyn Dimensional) -> bool {
        let mut size_a = self.size_xyz();
        size_a.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mut size_b = other.size_xyz();
        size_b.sort_by(|a, b| a.partial_cmp(b).unwrap());

        eq_tol(size_a[0], size_b[0]) && eq_tol(size_a[1], size_b[1]) && eq_tol(size_a[2], size_b[2])
    }
}

#[derive(Clone, Debug, Copy)]
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
    pub free_spaces: Vec<Space>,
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

        let mut previously_defragged: bool = false; // Only defrag once per free_spaces state, to avoid unecessary work
        let mut prev_item: Option<Item> = None; // Store the previously packed item, to compare against the current item and decide whether to defrag or not based on size/shape changes. TODO: This is a bit of a hack, but it seems to work well in testing and improves performance significantly with many items.

        for item in sorted_items {
            let mut best_fit: Option<(usize, [Dimension; 3], Space, Vec<Space>)> = None;

            // The algorithm is optimised to create 'channels' for packing items of similar size together,
            // but once we get to smaller items, these channels become too large and lead to wasted space when using
            // a first-fit strategy.
            // To avoid this, we want to defrag the spaces whenever the size/shape of the item we pack changes.
            prev_item.is_some().then(|| {
                if !item.is_same_size(prev_item.as_ref().unwrap()) {
                    free_spaces = Self::defrag(&mut free_spaces);
                    previously_defragged = true;
                }
            });

            // CASE 1: Iterate through all free spaces, to look for a space that fits the item
            for (index, space) in free_spaces.iter().enumerate() {

                if fits(space, &item) {
                    let (orientation, remainder) = Self::best_orientation(space, &item);
                    best_fit = Some((index, orientation, space.clone(), remainder));
                    break;
                }
            }

            // CASE 2: Couldn't find a space, so try to defrag the spaces by merging adjacent blocks.
            if best_fit.is_none() && !previously_defragged {

                free_spaces = Self::defrag(&mut free_spaces);
                previously_defragged = true;

                // Now we try to fit again
                for (index, space) in free_spaces.iter().enumerate() {
                    if fits(space, &item) {
                        let (orientation, remainder) = Self::best_orientation(space, &item);
                        best_fit = Some((index, orientation, space.clone(), remainder));
                        break;
                    }
                }
            }

            prev_item.replace(item.clone());

            // Found a space, so place the item and update the free spaces
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
                previously_defragged = false; // Reset defrag flag since the free spaces have changed

            } else { 
                // CASE 3: No possible spaces found! Try the next item.
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
            free_spaces,
            time_to_pack,
            bin_usage_percentage,
        }
    }

    fn best_orientation(space: &Space, item: &Item) -> ([Dimension; 3], Vec<Space>) {
        let b_dims_xyz = space.size_xyz();
        let mut b_dims = space.size.clone();
        let mut item_dims = item.size.clone();
        let mut remainder_blocks: Vec<Space> = Vec::new();

        // Sort dimensions ascending (shortest first)
        b_dims.sort_by(|a, b| a.length.partial_cmp(&b.length).unwrap());
        item_dims.sort_by(|a, b| a.length.partial_cmp(&b.length).unwrap());

        // Build the orientation of the item, side by side
        // First pass: Choose the shortest side of the box we can stack the item twice on its longest side,
        // Otherwise, try for an exact fit between the box and item dims
        let mut side_1_index: Option<usize> = None;
        for (i, b_dim) in b_dims.iter().enumerate() {
            if b_dim.length >= item_dims[2].length * 2.0 {
                side_1_index = Some(i);
                break;
            } 
            else if eq_tol(b_dim.length, item_dims[2].length) {
                side_1_index = Some(i);
                break;
            }
        }

        // If no suitable side was found, just go for the first fit
        if side_1_index.is_none() {
            for (i, b_dim) in b_dims.iter().enumerate() {
                if b_dim.length >= item_dims[2].length {
                    side_1_index = Some(i);
                    break;
                }
            }
        }

        // Orient the longest item's side to the chosen box side
        let mut dim_1 = item_dims[2].clone();
        dim_1.axis = b_dims[side_1_index.unwrap()].axis.clone(); 

        // Determine the orientation for the other two sides, preferring the combination that will have the largest singular volume
        let (side_2, side_3) = Self::get_side_2_side_3(&item_dims, &b_dims, side_1_index.unwrap());
        let mut dim_2 = item_dims[1].clone();
        let mut dim_3 = item_dims[0].clone();
        dim_2.axis = b_dims[side_2].axis.clone();
        dim_3.axis = b_dims[side_3].axis.clone();

        let orientation = [dim_3, dim_2, dim_1];
        let orientation_xyz: [f64; 3] = orientation.iter().fold([0.0, 0.0, 0.0], |mut acc, dim| {
            acc[dim.axis] = dim.length;
            acc
        });

        // First remaining space: Along the shortest side that we fit the item on. This can be of size 0, in which case it will be filtered out later.
        {
            let mut xyz = space.position_xyz.clone();
            let mut size = orientation_xyz.clone();
            xyz[dim_1.axis] += dim_1.length;
            size[dim_1.axis] = b_dims_xyz[dim_1.axis] - dim_1.length;

            remainder_blocks.push(Space {
                position_xyz: xyz,
                size: [
                    Dimension { length: size[0], axis: AxisSize::Width },
                    Dimension { length: size[1], axis: AxisSize::Height },
                    Dimension { length: size[2], axis: AxisSize::Depth },
                ],
            });
        }

        // Calculate how to split up the remaining space after occupation, of which there are two options:
        let block_2a: Space;
        let block_3a: Space;
        let block_2b: Space;
        let block_3b: Space;
        {
            let mut xyz = space.position_xyz.clone();
            let mut size = b_dims_xyz.clone();
            xyz[dim_3.axis] += item_dims[0].length;
            size[dim_3.axis] -= item_dims[0].length;
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
            let mut size = b_dims_xyz.clone();
            xyz[dim_2.axis] += item_dims[1].length;
            size[dim_2.axis] -= item_dims[1].length;

            size[dim_3.axis] = item_dims[0].length;
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
            let mut size = b_dims_xyz.clone();
            xyz[dim_2.axis] += item_dims[1].length;
            size[dim_2.axis] = space.size[dim_2.axis].length - item_dims[1].length;
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
            let mut size = b_dims_xyz.clone();
            xyz[dim_3.axis] += item_dims[0].length;
            size[dim_3.axis] = space.size[dim_3.axis].length - item_dims[0].length;

            size[dim_2.axis] = item_dims[1].length;
            block_3b = Space {
                position_xyz: xyz,
                size: [
                    Dimension { length: size[0], axis: AxisSize::Width },
                    Dimension { length: size[1], axis: AxisSize::Height },
                    Dimension { length: size[2], axis: AxisSize::Depth },
                ],
            };
        }

        // Select the option where block 2 and 3 are closest in size
        // This heuristic is used in the Shotput algo which claims to be 5-15% more accurate than
        // using 2a > 2b, but hasn't confirmed and tested here. However, 2a > 2b does lead to poor space
        // use in some cases like packing many cubes.
        if block_2a.volume() < block_2b.volume() {
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

        (orientation, filtered)
    }

    // Determines the rotation method by checking if the item MUST be rotated in a specific direction
    // based on size constraints, then returns the sides that leave the largest bulk volume in the box.
    // item_dims and box_dims are assumed to be sorted in ascending size.
    fn get_side_2_side_3(item_dims: &[Dimension], box_dims: &[Dimension], side_1: usize) -> (usize, usize) {

        let other_1 = (side_1 + 1) % 3; 
        let other_2 = (side_1 + 2) % 3;

        let (shorter_box_dim_index, longer_box_dim_index)  = if box_dims[other_1].length < box_dims[other_2].length {
            (other_1, other_2)
        } else {
            (other_2, other_1)
        };

        // Try to fit the second longest item dim on the next shortest box dim for best fit, otherwise we have to orient it the other way
        if item_dims[1].length <= box_dims[shorter_box_dim_index].length {
            (shorter_box_dim_index, longer_box_dim_index)
        } else {
            (longer_box_dim_index, shorter_box_dim_index)
        }
    }

    // Defragments free spaces by merging adjacent blocks that share a complete face
    // Returns the new defragmented spaces, sorted from smallest volume to largest
    fn defrag(free_spaces: &Vec<Space>) -> Vec<Space> {
        let mut new_spaces = free_spaces.clone();

        // Process in order of distance from origin
        new_spaces.sort_by(|a, b| {
            let total_distance_a: f64 = a.position_xyz.iter().sum();
            let total_distance_b: f64 = b.position_xyz.iter().sum();
            total_distance_a.partial_cmp(&total_distance_b).unwrap()
        });

        let mut adjacent_found: bool = true;
        while adjacent_found {
            adjacent_found = false;
            
            'outer: for i in 0..new_spaces.len() {
                for j in (i + 1)..new_spaces.len() {
                    let space_a = &new_spaces[i];
                    let space_b = &new_spaces[j];

                    for axis in [AxisSize::Width, AxisSize::Height, AxisSize::Depth] {
                        let a_min = space_a.position_xyz[axis];
                        let a_max = a_min + space_a.size[axis].length;
                        let b_min = space_b.position_xyz[axis];
                        let b_max = b_min + space_b.size[axis].length;

                        let other_axes: Vec<AxisSize> = [AxisSize::Width, AxisSize::Height, AxisSize::Depth]
                            .into_iter()
                            .filter(|&a| a != axis) 
                            .collect();
                        let face_dim_a1 = space_a.size[other_axes[0]].length;
                        let face_dim_a2 = space_a.size[other_axes[1]].length;
                        let face_dim_b1 = space_b.size[other_axes[0]].length;
                        let face_dim_b2 = space_b.size[other_axes[1]].length;

                        // CASE 1: Blocks are positioned along a same axis and share a face exactly can be merged.
                        if (eq_tol(a_max, b_min) || eq_tol(b_max, a_min)) && eq_tol(face_dim_a1, face_dim_b1) && eq_tol(face_dim_a2, face_dim_b2) 
                        {
                            let mut position_xyz = space_a.position_xyz.clone();
                            position_xyz[axis] = space_a.position_xyz[axis].min(space_b.position_xyz[axis]);
                            let mut size = space_a.size.clone();
                            size[axis] = Dimension {
                                length: space_a.size[axis].length + space_b.size[axis].length,
                                axis,
                            };

                            let merged = Space {
                                position_xyz,
                                size,
                            };
                            new_spaces.remove(j); 
                            new_spaces.remove(i);
                            new_spaces.insert(0, merged); // Spaces are processed in order of distance to origin, so we insert to first
                            adjacent_found = true;
                            break 'outer;
                        }
                        // CASE 2: Blocks that are adjacent but do not share a face exactly, might be able to be rearranged for an overall larger contiguous block
                        // WARNING! Did not finish when ran. Likely flawed, hypothesis is loops forming (A takes B, B takes C, C takes A)
                        // else if (eq_tol(a_max, b_min) || eq_tol(b_max, a_min)) && (eq_tol(face_dim_a1, face_dim_b1) || eq_tol(face_dim_a1, face_dim_b2))
                        // {
                        //     let (smaller_face_block, larger_face_block) = {
                        //         if face_dim_a1 < face_dim_b1 || face_dim_a1 < face_dim_b2 {
                        //             (space_a, space_b)
                        //         } else {
                        //             (space_b, space_a)
                        //         }
                        //     };

                        //     if smaller_face_block.volume() >= larger_face_block.volume() {
                        //         // Smaller face block takes over large face block, for a larger volume block.
                        //         let mut position_xyz = smaller_face_block.position_xyz.clone();
                        //         position_xyz[axis] = smaller_face_block.position_xyz[axis].min(larger_face_block.position_xyz[axis]);

                        //         let mut size = smaller_face_block.size.clone();
                        //         size[axis] = Dimension {
                        //             length: smaller_face_block.size[axis].length + larger_face_block.size[axis].length,
                        //             axis,
                        //         };

                        //         let merged = Space {
                        //             position_xyz,
                        //             size,
                        //         };

                        //         // Up to two remainder spaces created along the adjacent face's axis that is the shortest
                        //         let smaller_axis = if !eq_tol(face_dim_a1, face_dim_b1) {
                        //             other_axes[0]
                        //         } else {
                        //             other_axes[1]
                        //         };

                        //         let split_a_pos = larger_face_block.position_xyz.clone();
                        //         let mut split_a_size = larger_face_block.size.clone();
                        //         split_a_size[smaller_axis] = Dimension {
                        //             length: (larger_face_block.position_xyz[smaller_axis] - smaller_face_block.position_xyz[smaller_axis]).abs(),
                        //             axis: smaller_axis,
                        //         };
                        //         let split_a = Space {
                        //             position_xyz: split_a_pos,
                        //             size: split_a_size,
                        //         };

                        //         let mut split_b_pos = larger_face_block.position_xyz.clone();
                        //         split_b_pos[smaller_axis] = smaller_face_block.position_xyz[smaller_axis];
                        //         let mut split_b_size = larger_face_block.size.clone();
                        //         split_b_size[smaller_axis] = Dimension {
                        //             length: (larger_face_block.position_xyz[smaller_axis] - smaller_face_block.position_xyz[smaller_axis] + smaller_face_block.size[smaller_axis].length).abs(),
                        //             axis: smaller_axis,
                        //         };
                        //         let split_b = Space {
                        //             position_xyz: split_b_pos,
                        //             size: split_b_size,
                        //         };


                        //         new_spaces.remove(j); 
                        //         new_spaces.remove(i);
                        //         new_spaces.push(merged);

                        //         if !eq_tol(split_a.volume(), 0.0) {
                        //             new_spaces.push(split_a);
                        //         }
                        //         if !eq_tol(split_b.volume(), 0.0) {
                        //             new_spaces.push(split_b);
                        //         }

                        //         adjacent_found = true;
                        //         break 'outer;
                        //     }

                        // }
                    }

                }
            }   

        }

        // Make sure they're arranged from smallest volume to largest for first-fit
        new_spaces.sort_by(|a, b| {
            a.volume().partial_cmp(&b.volume()).unwrap()
        });

        new_spaces
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
    let mut sorted_size_a = container.size_xyz();
    sorted_size_a.sort_by( |a, b|{
        b.partial_cmp(&a).unwrap() }); 

    let mut sorted_size_b = to_fit.size_xyz();
    sorted_size_b.sort_by( |a, b|{
        b.partial_cmp(&a).unwrap() }); 

    sorted_size_a[0] >= sorted_size_b[0] && 
    sorted_size_a[1] >= sorted_size_b[1] && 
    sorted_size_a[2] >= sorted_size_b[2]
}