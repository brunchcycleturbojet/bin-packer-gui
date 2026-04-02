import "./style/App.css";

import { invoke } from "@tauri-apps/api/core";
import {Bin, Item} from "./BinData";
import { useState } from "react";

interface PackerControlsProps {
  bin: Bin;
  items: Item[];
  onItemsPacked: (items: Item[]) => void;
  onBinPacked: (bin: Bin) => void;
}

function PackerControls({ bin, items, onItemsPacked, onBinPacked }: PackerControlsProps) {
  const datasets = [
    { name: "Simple Data", data: SimpleData },
    { name: "Test Data", data: TestData },
    { name: "HLJ Data", data: HLJData },
    { name: "HLJ Rusty", data: HLJRusty },
  ];

  const [currentDatasetIndex, setCurrentDatasetIndex] = useState(0);

  function cycleDataset() {
    setCurrentDatasetIndex((prevIndex) => (prevIndex + 1) % datasets.length);
  }

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

    const payload = datasets[currentDatasetIndex].data;
    

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
    <span>
      <h2>{datasets[currentDatasetIndex].name}</h2>
      <form
          className="row"
          onSubmit={(e) => {
            e.preventDefault();
            pack_bin();
          }}
        >
          <button type="submit">Pack</button>
          <button type="button" onClick={cycleDataset}>Next Dataset</button>
      </form>
    </span>

  );
}

export default PackerControls;

const SimpleData = {
  "bin": {
    "width": 3.0,
    "height": 5.0,
    "depth": 3.0
  },
  "items": [
    {
      "id": 0,
      "name": "example1",
      "x": 0,
      "y": 0,
      "z": 0,
      "width": 2.0,
      "height": 1.0,
      "depth": 3.0,
    },
    {
      "id": 1,
      "name": "example1",
      "x": 0,
      "y": 0,
      "z": 0,
      "width": 1.0,
      "height": 3.0,
      "depth": 2.0,
    },
    {
      "id": 2,
      "name": "example1",
      "x": 0,
      "y": 0,
      "z": 0,
      "width": 1.0,
      "height": 1.0,
      "depth": 5.0,
    },
    {
      "id": 3,
      "name": "example1",
      "x": 0,
      "y": 0,
      "z": 0,
      "width": 1.0,
      "height": 1.0,
      "depth": 4.0,
    },
  ]
};

const TestData = {
  "bin": {
    "width": 3,
    "height": 2,
    "depth": 1
  },
  "items": [
    {
      "id": 0,
      "name": "example1",
      "x": 0,
      "y": 0,
      "z": 0,
      "width": 1,
      "height": 0.5,
      "depth": 1,
    },
    {
      "id": 1,
      "name": "example1",
      "x": 0,
      "y": 0,
      "z": 0,
      "width": 2,
      "height": 1,
      "depth": 1,
    },
    {
      "id": 2,
      "name": "example1",
      "x": 0,
      "y": 0,
      "z": 0,
      "width": 1,
      "height": 0.4,
      "depth": 1,
    },
    {
      "id": 3,
      "name": "example1",
      "x": 0,
      "y": 0,
      "z": 0,
      "width": 1,
      "height": 1,
      "depth": 1,
    },
    {
      "id": 4,
      "name": "example1",
      "x": 0,
      "y": 0,
      "z": 0,
      "width": 3,
      "height": 0.2,
      "depth": 1,
    },
  ]
};

const HLJData = {
  "bin": {
    "width": 48,
    "height": 35,
    "depth": 33
  },
  "items": [
    {
      "id": 0,
      "name": "liger panzer",
      "x": 0,
      "y": 0,
      "z": 0,
      "width": 40,
      "height": 14,
      "depth": 33,
    },
    {
      "id": 1,
      "name": "arhan",
      "x": 0,
      "y": 0,
      "z": 0,
      "width": 30,
      "height": 11,
      "depth": 19,
    },
    {
      "id": 2,
      "name": "new arhan",
      "x": 0,
      "y": 0,
      "z": 0,
      "width": 30,
      "height": 12,
      "depth": 21,
    },
    {
      "id": 3,
      "name": "gqux",
      "x": 0,
      "y": 0,
      "z": 0,
      "width": 30,
      "height": 7,
      "depth": 19,
    },
        {
      "id": 4,
      "name": "gqux",
      "x": 0,
      "y": 0,
      "z": 0,
      "width": 30,
      "height": 7,
      "depth": 19,
    },
            {
      "id": 5,
      "name": "gqux",
      "x": 0,
      "y": 0,
      "z": 0,
      "width": 30,
      "height": 7,
      "depth": 19,
    },
                {
      "id": 6,
      "name": "gqux",
      "x": 0,
      "y": 0,
      "z": 0,
      "width": 30,
      "height": 7,
      "depth": 19,
    },
  ]
};

const HLJRusty = {
  "bin": {
    "width": 48,
    "height": 35,
    "depth": 33
  },
  "items": [
    {
      "id": 0,
      "name": "rusty",
      "x": 0,
      "y": 0,
      "z": 0,
      "width": 40,
      "height": 14,
      "depth": 33,

    },
    {
      "id": 0,
      "name": "rusty",
      "x": 0,
      "y": 0,
      "z": 0,
      "width": 40,
      "height": 14,
      "depth": 33,
    },
    {
      "id": 2,
      "name": "yukikaze mave",
      "x": 0,
      "y": 0,
      "z": 0,
      "width": 23,
      "height": 14,
      "depth": 4,
    },
  ]
};