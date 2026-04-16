#[cfg(test)]
mod tests {
    use crate::packer::{AxisSize, Bin, BinPacker3D, Dimension, Item};

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
                id: item_id,
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
                id: item_id,
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
}
