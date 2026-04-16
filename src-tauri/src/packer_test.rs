#[cfg(test)]
mod tests {
    use crate::packer::{AxisSize, Bin, BinPacker3D, Dimension, Dimensional, Item};

    #[test]
    fn test_stacking() {
        // Verify algorithm prefers to stack the longest item dimension along the bin's shortest dimension
        let bin = Bin {
            width: 48.0,
            height: 35.0,
            depth: 33.0,
        };

        let mut items: Vec<Item> = Vec::new();
        let mut item_id = 0;
        for _ in 0..6 {
            items.push(Item {
                shape_id: item_id,
                name: "item".to_string(),
                position_xyz: [0.0, 0.0, 0.0],
                size: [
                    Dimension { length: 12.0, axis: AxisSize::Width },
                    Dimension { length: 19.5, axis: AxisSize::Height },
                    Dimension { length: 31.0, axis: AxisSize::Depth },
                ],
            });
            item_id += 1;
        }

        let result = BinPacker3D::pack(bin, items);

        assert_eq!(result.placed.len(), 6, "All items should be placed in the bin");
        assert_eq!(result.unplaced.len(), 0, "No items should be unplaced");
        assert_items_within_bin_bounds(&result.placed, &result.bin);

        assert!(
            result.placed[0..4].iter().all(|item| item.position_xyz[1] == 0.0 && item.position_xyz[2] == 0.0),
            "First four items should have been stacked along the x-axis"
        );
        assert!(
            result.placed[4..6].iter().all(|item| item.position_xyz[1] == 19.5 && item.position_xyz[2] == 0.0),
            "Second last item should have been placed on top of the first four, at x=0.0"
        );
        assert_eq!(
            result.placed[5].position_xyz[0], 19.5, 
            "Last item should have been placed next to the second last item, at x=19.5"
        );

    }

    #[test]
    fn test_pack_all_items_in_bin() {
        let bin = Bin {
            width: 10.0,
            height: 10.0,
            depth: 10.0,
        };
        let items: Vec<Item> = vec![
            Item {
                shape_id: 0,
                name: "item_1".to_string(),
                position_xyz: [0.0, 0.0, 0.0],
                size: [
                    Dimension { length: 5.0, axis: AxisSize::Width },
                    Dimension { length: 5.0, axis: AxisSize::Height },
                    Dimension { length: 5.0, axis: AxisSize::Depth },
                ],
            },
            Item {
                shape_id: 1,
                name: "item_2".to_string(),
                position_xyz: [0.0, 0.0, 0.0],
                size: [
                    Dimension { length: 10.0, axis: AxisSize::Width },
                    Dimension { length: 5.0, axis: AxisSize::Height },
                    Dimension { length: 10.0, axis: AxisSize::Depth },
                ],
            },
            Item {
                shape_id: 2,
                name: "item_3".to_string(),
                position_xyz: [0.0, 0.0, 0.0],
                size: [
                    Dimension { length: 10.0, axis: AxisSize::Width },
                    Dimension { length: 5.0, axis: AxisSize::Height },
                    Dimension { length: 5.0, axis: AxisSize::Depth },
                ],
            },
        ];

        let result = BinPacker3D::pack(bin, items);

        // Assert that all items were placed
        assert_eq!(result.placed.len(), 3, "Both items should be placed in the bin");
        assert_eq!(result.unplaced.len(), 0, "No items should be unplaced");
        assert_items_within_bin_bounds(&result.placed, &result.bin);

    }

    #[test]
    fn test_many_cubes() {
        let bin = Bin {
            width: 50.0,
            height: 14.0,
            depth: 50.0,
        };

        let mut items: Vec<Item> = Vec::new();
        let mut item_id = 0;
        for _ in 0..1024 {
            items.push(Item {
                shape_id: item_id,
                name: "small".to_string(),
                position_xyz: [0.0, 0.0, 0.0],
                size: [
                    Dimension { length: 2.0, axis: AxisSize::Width },
                    Dimension { length: 2.0, axis: AxisSize::Height },
                    Dimension { length: 2.0, axis: AxisSize::Depth },
                ],
            });
            item_id += 1;
        }
        for _ in 0..1024 {
            items.push(Item {
                shape_id: item_id,
                name: "medium".to_string(),
                position_xyz: [0.0, 0.0, 0.0],
                size: [
                    Dimension { length: 3.0, axis: AxisSize::Width },
                    Dimension { length: 3.0, axis: AxisSize::Height },
                    Dimension { length: 3.0, axis: AxisSize::Depth },
                ],
            });
            item_id += 1;
        }

        let result = BinPacker3D::pack(bin, items);

        // Optimal result should pack all medium boxes, then fill the rest of the space with small boxes
        assert!(result.placed.len() == 1943, "Expected 1943 items to be placed, but got {}", result.placed.len());
        assert!(result.unplaced.len() == 105, "Expected 105 items to be unplaced, but got {}", result.unplaced.len());
        assert_items_within_bin_bounds(&result.placed, &result.bin);

    }


    // ----------------------------------------------------------------
    // Helper functions

    fn assert_items_within_bin_bounds(items: &[Item], bin: &Bin) {
        for item in items {
            assert!(
                item.position_xyz[0] + item.size_xyz()[0] <= bin.width,
                "Item {} extends beyond bin width",
                item.shape_id
            );
            assert!(
                item.position_xyz[1] + item.size_xyz()[1] <= bin.height,
                "Item {} extends beyond bin height",
                item.shape_id
            );
            assert!(
                item.position_xyz[2] + item.size_xyz()[2] <= bin.depth,
                "Item {} extends beyond bin depth",
                item.shape_id
            );
        }
    }
}