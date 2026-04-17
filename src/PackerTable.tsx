import "./style/App.css";
import { Bin, Item, FreeSpace, LoadOutput, PackerOutput } from "./BinData";
import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open, save } from "@tauri-apps/plugin-dialog";

interface PackerTableProps {
  bin: Bin;
  items: Item[];
  onItemsPacked?: (items: Item[]) => void;
  onBinPacked?: (bin: Bin) => void;
  onFreeSpacesPacked?: (spaces: FreeSpace[]) => void;
}

function PackerTable({ bin, items: _items, onItemsPacked, onBinPacked, onFreeSpacesPacked }: PackerTableProps) {
  const [getPendingBin, setPendingBin] = useState<Bin>(bin);
  const [getPendingItems, setPendingItems] = useState<Item[]>([]);

  // Run packing algo
  async function pack_bin() {
    const payload = {
      bin: {
        width: getPendingBin.width,
        height: getPendingBin.height,
        depth: getPendingBin.depth,
      },
      items: getPendingItems,
    };

    const json = JSON.stringify(payload);
    const result: string = await invoke("pack_bin", { json });

    if (!result) {
      console.error("pack_bin returned no data");
    } else {
      const parsedJSON: PackerOutput = JSON.parse(result);
      const newBin: Bin = parsedJSON.bin;
      const newItems: Item[] = parsedJSON.items;
      const newFreeSpaces: FreeSpace[] = parsedJSON.free_spaces;

      onBinPacked?.(newBin);
      onItemsPacked?.(newItems);
      onFreeSpacesPacked?.(newFreeSpaces);
    }
  }

  function renderPackButton() {
    return (
      <button onClick={() => pack_bin()}>Pack</button>
    );
  }

  // Load a bin from file
  async function loadBinFromFile() {
    try {
      const filePath = await open({
        filters: [{ name: "JSON", extensions: ["json"] }],
        directory: false,
      });

      if (!filePath) return; // User cancelled

      console.log("Selected file:", filePath);
      const result: string = await invoke("load_bin_and_items", { filePath });
      if (!result) {
        alert("Failed to load file");
        return;
      }

      const parsedJSON: LoadOutput = JSON.parse(result);
      setPendingBin(parsedJSON.pack_input.bin);
      setPendingItems(parsedJSON.pack_input.items); 

      onBinPacked?.(parsedJSON.pack_result.bin);
      onItemsPacked?.(parsedJSON.pack_result.items);
      onFreeSpacesPacked?.(parsedJSON.pack_result.free_spaces);

    } catch (error) {
      console.error("Error loading file:", error);
      alert("Error loading file");
    }
  }

  // Save a bin to file
  async function saveBinToFile() {
    try {
      const filePath = await save({
        filters: [{ name: "JSON", extensions: ["json"] }],
      });

      if (!filePath) return; // User cancelled

      const payload = {
        bin: {
          width: bin.width,
          height: bin.height,
          depth: bin.depth,
        },
        items: getPendingItems,
      };

      const json = JSON.stringify(payload);

      const result: string = await invoke("save_bin_and_items", { json, filePath });
      if (result) {
        alert(`Saved to ${result}`);
      } else {
        alert("Failed to save file");
      }
    } catch (error) {
      console.error("Error saving file:", error);
      alert("Error saving file");
    }
  }

  // Add new item, with default parameters
  function addItem() {
    const newItem: Item = {
      shape_id: Math.max(...getPendingItems.map(i => i.shape_id), 0) + 1,
      name: `Item ${getPendingItems.length + 1}`,
      x: 0,
      y: 0,
      z: 0,
      width: 1,
      height: 1,
      depth: 1,
      quantity: 1,
    };

    setPendingItems([...getPendingItems, newItem]);
  }
  
  function renderAddItemButton() {
    return (
      <button onClick={() => addItem()}>Add Item</button>
    );
  }

  // Update a specific item
  function updateItem(shape_id: number, updates: Partial<Item>) {
    const updatedItems = getPendingItems.map(item =>
      item.shape_id === shape_id ? { ...item, ...updates } : item
    );
    setPendingItems(updatedItems);
  }

  // Delete item
  function renderRemoveButton(shape_id: number) {
    return (
      <button onClick={() => setPendingItems(getPendingItems.filter(item => item.shape_id !== shape_id))}>Remove</button>
    );
  }

  // Update item(s)

  function renderDimensionInput(shape_id: number, width: number, height: number, depth: number) {
    const MIN_VALUE = 0.01;
    const MAX_VALUE = 100.0;

    const renderInput = (dim: number, field: "width" | "height" | "depth") => {
      return(
        <input
            type="number"
            value={dim}
            className="input"
            onChange={(e) => updateItem(shape_id, { [field]: parseFloat(e.target.value) })}
            onBlur={(e) => {
              // Ensure value is within bounds on loss of focus
              let value = parseFloat(e.target.value);
              if (isNaN(value)) value = 1.0; // Default to 1 if empty
              value = Math.max(MIN_VALUE, Math.min(MAX_VALUE, value));
              updateItem(shape_id, { [field]: value });
            }}
            min={0} // Feels better to scroll to 0, rather than to 0.01 then have to remove the decimal if unwanted
            max={MAX_VALUE}
            step={1.0}
          />
      );
    }

    return (
      <div className="dimension-wrapper">
        {renderInput(width, "width")}
        <span>×</span>
        {renderInput(height, "height")}
        <span>×</span>
        {renderInput(depth, "depth")}
      </div>
    );
  }

  const binDescription = `(${getPendingBin.width}×${getPendingBin.height}×${getPendingBin.depth})`;
  return (
    <>
      <div>
        {renderPackButton()}
        <button onClick={() => loadBinFromFile()}>Load</button>
        <button onClick={() => saveBinToFile()}>Save</button>
        {renderAddItemButton()}
        <h2>Bin: {binDescription}</h2>
      </div>
      <div className="table-container">
      <table>
        <tbody>
          <tr>
            <th>{/* TODO: Toggle visiblity */}</th>
            <th>{/* TODO: Display colour */}</th>
            <th>Name</th>
            <th>Size</th>
            <th>Qty</th>
            <th>{/* Column for remove item button */}</th>
          </tr>
          {getPendingItems && getPendingItems.length > 0 ? (
            getPendingItems.map((item) => (
              <tr key={item.shape_id}>
                <td><input type="checkbox" name={`item_${item.shape_id}`} value={`${item.shape_id}`} /></td>
                <td>TODO{/* Colour display */}</td>
                <td>
                  <input
                    type="text"
                    value={item.name}
                    onChange={(e) => updateItem(item.shape_id, { name: e.target.value })}
                  />
                </td>
                <td>
                  {renderDimensionInput(item.shape_id, item.width, item.height, item.depth)}
                </td>
                <td>
                  <input
                    type="number"
                    value={item.quantity}
                    onChange={(e) => updateItem(item.shape_id, { quantity: parseInt(e.target.value) })}
                    className="input"
                  />
                </td>
                <td>{renderRemoveButton(item.shape_id)}</td>
              </tr>
            ))): (
              <>
              <tr>
                <td colSpan={6}>No items loaded</td>
              </tr>
              </>
            )
          }

          {/* <tr> <td colSpan={6}><button onClick={() => addItem()}>Add Item</button></td> </tr> */}
        </tbody>

      </table>
      </div>
    </>
  );
}

export default PackerTable;