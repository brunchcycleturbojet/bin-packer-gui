import "./style/App.css";

import { invoke } from "@tauri-apps/api/core";
import {Bin, Item} from "./BinData";

interface PackerControlsProps {
  bin: Bin;
  items: Item[];
  onItemsPacked: (items: Item[]) => void;
  onBinPacked: (bin: Bin) => void;
}

function PackerControls({ bin, items, onItemsPacked, onBinPacked }: PackerControlsProps) {

  // Run packing algo, position items inside bin
  async function pack_bin() {
    // const payload = {
    //   bin: {
    //     width: bin.width,
    //     height: bin.height,
    //     depth: bin.depth,
    //   },
    //   items: items,
    // };

    const payload = ExampleData;

    const json = JSON.stringify(payload);
    const result: string = await invoke("pack_bin", { json });

    if (!result) {
        console.error("pack_bin returned no data");
    } else {
        const parsedJSON = JSON.parse(result);
        const newBin: Bin = parsedJSON.bin;
        const newItems: Item[] = parsedJSON.items;

        console.log(newItems);

        onBinPacked(newBin);
        onItemsPacked(newItems);
    }
  }

  return (
    <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          pack_bin();
        }}
      >
        <button type="submit">Pack</button>
    </form>
  );
}

export default PackerControls;

const ExampleData = {
  "bin": {
    "width": 3,
    "height": 5,
    "depth": 3
  },
  "items": [
    {
      "id": 0,
      "name": "example1",
      "x": 0,
      "y": 0,
      "z": 0,
      "width": 2,
      "height": 2,
      "depth": 2
    },
    {
      "id": 1,
      "name": "ex2",
      "x": 0,
      "y": 0,
      "z": 0,
      "width": 3,
      "height": 2,
      "depth": 3
    },
    {
      "id": 2,
      "name": "ex3",
      "x": 0,
      "y": 0,
      "z": 0,
      "width": 1,
      "height": 1,
      "depth": 1
    },
    {
      "id": 3,
      "name": "ex4",
      "x": 0,
      "y": 0,
      "z": 0,
      "width": 2,
      "height": 1,
      "depth": 2
    }
  ]
};