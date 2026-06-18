import { Bin, Item, FreeSpace, LoadOutput, PackerOutput } from "../bin-data";
import { invoke } from "@tauri-apps/api/core";
import { open, save } from "@tauri-apps/plugin-dialog";
import { useAppStore } from "../store";
import { Button } from "./ui/button";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "./ui/table";
import { RiAddBoxLine, RiDeleteBinLine, RiEyeLine, RiImportFill, RiPaletteLine, RiSave3Fill } from "@remixicon/react";
import { Input } from "#components/ui/input";
import { Checkbox } from "#components/ui/checkbox";
import { Field } from "#components/ui/field";
import { HexColorPicker } from "react-colorful";
import { Popover, PopoverContent, PopoverTrigger } from "#components/ui/popover";
import { ButtonGroup } from "#components/ui/button-group";
import { Tooltip, TooltipContent, TooltipTrigger } from "#components/ui/tooltip";
import { ScrollArea } from "./ui/scroll-area";

function PackerParams() {
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
        // alert(`Saved to ${result}`);
        // TODO: Popup with saved confirmation
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
            className="text-center [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none"
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
        <div className="flex items-center text-2xl grow pr-5 text-nowrap" >The Big Bin</div>
        <Field orientation={"horizontal"} className="grow-2 mr-10">
          {renderInput(width, "width")}
          <span>×</span>
          {renderInput(height, "height")}
          <span>×</span>
          {renderInput(depth, "depth")}
          <span className="text-xs h-full flex flex-col justify-end text-muted-foreground">cm</span>
        </Field>
      </div>
    );
  }

  return (
    <div className="h-full flex flex-col">
      <div className="flex h-20 w-full justify-around pb-2 pt-4 pr-6 pl-6 border-t-2">
        <div className="w-full">
          {renderBinDimensionInput(pendingBin.width, pendingBin.height, pendingBin.depth)}
        </div>
        <ButtonGroup>
          <Tooltip>
            <TooltipTrigger asChild>
              <Button type="button" size="icon-lg" onClick={() => loadBinFromFile()}>
                <RiImportFill className="size-6" />
              </Button>
            </TooltipTrigger>
            <TooltipContent>Load</TooltipContent>
          </Tooltip>
          <Tooltip>
            <TooltipTrigger asChild>
              <Button type="button" size="icon-lg" onClick={() => saveBinToFile()}>
                <RiSave3Fill className="size-6"/>
              </Button>
            </TooltipTrigger>
            <TooltipContent>Save as</TooltipContent>
          </Tooltip>
          <Tooltip>
            <TooltipTrigger asChild>
              <Button type="button" size="icon-lg" className="" onClick={() => addItem()}>
                <RiAddBoxLine className="size-6"/>
              </Button>
            </TooltipTrigger>
            <TooltipContent>Add item</TooltipContent>
          </Tooltip>
          <Button type="button" size="lg" className="flex items-center" variant="submit" onClick={() => pack_bin()}>
            <b className="pt-0.5">Pack!</b>
          </Button>
        </ButtonGroup>
      </div>
      <ScrollArea className="grow min-h-0">
        <Table className="h-full w-full relative">
          <TableHeader className="sticky top-0 bg-secondary">
            <TableRow>
              <TableHead><div className="flex items-center justify-center" ><RiEyeLine size={18} /></div></TableHead>
              <TableHead><div className="flex items-center justify-center" ><RiPaletteLine size={18} /></div></TableHead>
              <TableHead className="text-center">Name</TableHead>
              <TableHead className="text-center">Size (cm)</TableHead>
              <TableHead className="text-center w-20">Qty</TableHead>
              <TableHead>{/* Column for remove item button */}</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {pendingItems && pendingItems.length > 0 ? (pendingItems.map((item) => (
              <TableRow key={item.shape_id}>
                <TableCell> {/* Visibility toggle, to implement */}
                  <Checkbox checked={true} /> 
                </TableCell>
                <TableCell> {/* Colour display, to implement */}
                  <Popover>
                    <PopoverTrigger className="flex items-center justify-center">
                      <div className="bg-blue-500 hover:bg-blue-700 text-white size-4.5 border border-blue-700 rounded"/>
                    </PopoverTrigger>
                    <PopoverContent className="flex items-center w-fit p-5">
                      <HexColorPicker />
                      <Button>Reset</Button>
                    </PopoverContent>
                  </Popover>
                </TableCell>
                <TableCell> {/* Item name */}
                  <Input
                    type="text"
                    value={item.name}
                    onChange={(e) => updateItem(item.shape_id, { name: e.target.value })}
                    className="text-ellipsis"
                  />
                </TableCell>
                <TableCell> {/* Dimensions */}
                  {renderDimensionInput(item.shape_id, item.width, item.height, item.depth)}
                </TableCell>
                <TableCell> {/* Quantity */}
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
      </ScrollArea>
    </div>
  );
}

export default PackerParams;