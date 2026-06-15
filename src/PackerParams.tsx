import "./style/App.css";
import { Bin, Item, FreeSpace, LoadOutput, PackerOutput } from "./binData";
import { invoke } from "@tauri-apps/api/core";
import { open, save } from "@tauri-apps/plugin-dialog";
import { useAppStore, TabState } from "./store";
import { Button } from "./components/ui/button";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "./components/ui/table";
import { RiDeleteBinLine, RiEyeLine, RiPaletteLine } from "@remixicon/react";
import { Input } from "#components/ui/input";
import { Checkbox } from "#components/ui/checkbox";
import { ScrollArea, ScrollBar } from "#components/ui/scroll-area";
import { Field } from "#components/ui/field";

function PackerTable() {
  const { 
    bin, 
    pendingBin, 
    pendingItems, 
    updateBin, 
    updateItems, 
    updateFreeSpaces, 
    updatePendingBin, 
    updatePendingItems,
    updateLastPackedItems,
    updateLastUnpackedItems,
    setActiveTab,
  } = useAppStore();

  // Run packing algo
  async function pack_bin() {
    const payload = {
      bin: {
        width: pendingBin.width,
        height: pendingBin.height,
        depth: pendingBin.depth,
      },
      items: pendingItems,
    };

    const json = JSON.stringify(payload);
    const result: string = await invoke("pack_bin", { json });

    if (!result) {
      console.error("pack_bin returned no data");
    } else {
      const parsedJSON: PackerOutput = JSON.parse(result);
      const newBin: Bin = parsedJSON.bin;
      const newItems: Item[] = parsedJSON.item_pos;
      const newFreeSpaces: FreeSpace[] = parsedJSON.free_space_pos;
      const placedItems: Item[] = parsedJSON.placed_items || [];
      const unpackedItems: Item[] = parsedJSON.unpacked_items || [];

      updateBin(newBin);
      updateItems(newItems);
      updateFreeSpaces(newFreeSpaces);
      updateLastPackedItems(placedItems);
      updateLastUnpackedItems(unpackedItems);

      // Move to results tab on packing
      setActiveTab(TabState.Result);
    }
  }

  // Load a bin from file
  async function loadBinFromFile() {
    try {
      const filePath = await open({
        filters: [{ name: "JSON", extensions: ["json"] }],
        directory: false,
      });

      if (!filePath) return; // User cancelled

      const result: string = await invoke("load_bin_and_items", { filePath });
      if (!result) {
        alert("Failed to load file");
        return;
      }

      const parsedJSON: LoadOutput = JSON.parse(result);
      updatePendingBin(parsedJSON.pack_input.bin);
      updatePendingItems(parsedJSON.pack_input.items); 

      const placedItems = parsedJSON.pack_result.placed_items || [];
      const unpackedItems = parsedJSON.pack_result.unpacked_items || [];
      updateBin(parsedJSON.pack_result.bin);
      updateItems(parsedJSON.pack_result.item_pos);
      updateFreeSpaces(parsedJSON.pack_result.free_space_pos);
      updateLastPackedItems(placedItems);
      updateLastUnpackedItems(unpackedItems);

      // Move to results tab after loading and packing
      setActiveTab(TabState.Result);
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
        items: pendingItems,
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
      shape_id: Math.max(...pendingItems.map(i => i.shape_id), 0) + 1,
      name: `Item ${pendingItems.length + 1}`,
      x: 0,
      y: 0,
      z: 0,
      width: 1,
      height: 1,
      depth: 1,
      quantity: 1,
    };

    updatePendingItems([...pendingItems, newItem]);
  }

  // Delete item
  function renderRemoveButton(shape_id: number) {
    return (
      <Button 
        variant="destructive" 
        size="sm" 
        onClick={() => updatePendingItems(pendingItems.filter(item => item.shape_id !== shape_id))}>
        <RiDeleteBinLine />
      </Button>
    );
  }

  // Update item(s)
  function updateItem(shape_id: number, updates: Partial<Item>) {
    const updatedItems = pendingItems.map(item =>
      item.shape_id === shape_id ? { ...item, ...updates } : item
    );
    updatePendingItems(updatedItems);
  }

  function renderDimensionInput(shape_id: number, width: number, height: number, depth: number) {
    const MIN_VALUE = 0.01;
    const MAX_VALUE = 100.0;

    const renderInput = (dim: number, field: "width" | "height" | "depth") => {
      return(
        <Input
            type="number"
            value={dim}
            className="text-center [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none"
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
      <div className="flex items-center">
        {renderInput(width, "width")}
        <span className="mr-1 ml-1" >×</span>
        {renderInput(height, "height")}
        <span className="mr-1 ml-1">×</span>
        {renderInput(depth, "depth")}
      </div>
    );
  }

  // Update bin
  function updatePendingBinLocal(updated: Partial<Bin>) {
      updatePendingBin({ ...pendingBin, ...updated});
  }
  function renderBinDimensionInput(width: number, height: number, depth: number) {
    const MIN_VALUE = 0.01;
    const MAX_VALUE = 100.0;

    const renderInput = (dim: number, field: "width" | "height" | "depth") => {
      return(
        <Input
            type="number"
            value={dim}
            className=""
            onChange={(e) => updatePendingBinLocal({ [field]: parseFloat(e.target.value) })}
            onBlur={(e) => {
              // Ensure value is within bounds on loss of focus
              let value = parseFloat(e.target.value);
              if (isNaN(value)) value = 1.0; // Default to 1 if empty
              value = Math.max(MIN_VALUE, Math.min(MAX_VALUE, value));
              updatePendingBinLocal({ [field]: value });
            }}
            min={0} // Feels better to scroll to 0, rather than to 0.01 then have to remove the decimal if unwanted
            max={MAX_VALUE}
            step={1.0}
          />
      );
    }

    return (
      <div className="flex flex-row w-full justify-around pb-2">
        <span className="grow pl-10 pr-10" >BinName </span>
        <Field orientation={"horizontal"} className="grow-2 mr-10">
          {renderInput(width, "width")}
          <span>×</span>
          {renderInput(height, "height")}
          <span>×</span>
          {renderInput(depth, "depth")}
        </Field>
      </div>
    );
  }

  return (
    <>
      <div id="tableTitleControls" className="flex flex-col w-full justify-center pb-2">
        <div>
          {renderBinDimensionInput(pendingBin.width, pendingBin.height, pendingBin.depth)}
        </div>
        <div className="flex justify-around">
          <Button onClick={() => loadBinFromFile()}>Load</Button>
          <Button onClick={() => saveBinToFile()}>Save as</Button>
          <Button onClick={() => addItem()}>Add Item</Button>
          <Button variant="submit" onClick={() => pack_bin()}>Pack</Button>
        </div>
      </div>
      <div className="h-full w-full">
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead><div className="flex items-center justify-center" ><RiEyeLine size={18} /></div></TableHead>
              <TableHead><div className="flex items-center justify-center" ><RiPaletteLine size={18} /></div></TableHead>
              <TableHead className="text-center">Name</TableHead>
              <TableHead className="text-center">Size (cm)</TableHead>
              <TableHead className="text-center">Qty</TableHead>
              <TableHead>{/* Column for remove item button */}</TableHead>
            </TableRow>
          </TableHeader>
            <TableBody>
              {pendingItems && pendingItems.length > 0 ? (
                pendingItems.map((item) => (
                  <TableRow key={item.shape_id}>
                    <TableCell> <Checkbox /> {/* Visibility toggle, to implement */}</TableCell>
                    <TableCell>TD{/* Colour display, to implement */}</TableCell>
                    <TableCell>
                      <Input
                        type="text"
                        value={item.name}
                        onChange={(e) => updateItem(item.shape_id, { name: e.target.value })}
                        className="text-ellipsis"
                      />
                    </TableCell>
                    <TableCell>
                      {renderDimensionInput(item.shape_id, item.width, item.height, item.depth)}
                    </TableCell>
                    <TableCell>
                      <Input
                        type="number"
                        value={item.quantity}
                        onChange={(e) => updateItem(item.shape_id, { quantity: parseInt(e.target.value) })}
                        className="text-ellipsis text-right [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none"
                      />
                    </TableCell>
                    <TableCell>{renderRemoveButton(item.shape_id)}</TableCell>
                  </TableRow>
                ))
              ) : (
                <TableRow>
                  <TableCell colSpan={6} className="text-center">
                    No items
                  </TableCell>
                </TableRow>
              )}
            </TableBody>
        </Table>
      </div>
    </>
  );
}

export default PackerTable;