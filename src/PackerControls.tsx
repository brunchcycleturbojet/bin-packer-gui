import "./style/App.css";

import { invoke } from "@tauri-apps/api/core";
import {Bin, Item, FreeSpace} from "./BinData";
import { useState, useEffect } from "react";

interface PackerControlsProps {
  bin: Bin;
  items: Item[];
  onItemsPacked: (items: Item[]) => void;
  onBinPacked: (bin: Bin) => void;
  onFreeSpacesPacked: (spaces: FreeSpace[]) => void;
}

function PackerControls({ bin, items: _items, onItemsPacked, onBinPacked, onFreeSpacesPacked }: PackerControlsProps) {

  const [currentDatasetIndex, setCurrentDatasetIndex] = useState(0);
  const [testSet, setTestSet] = useState(Cubes);
  const datasets = [
    { name: "Cubes", data: testSet },
    { name: "Simple Data", data: SimpleData },
    { name: "Test Data", data: TestData },
    { name: "HLJ Data", data: HLJData },
    { name: "HLJ Data 2", data: HLJData2 },
  ];

  useEffect(() => {
    pack_bin();
  }, [currentDatasetIndex, testSet]);

  function cycleDataset() {
    setCurrentDatasetIndex((prevIndex) => (prevIndex + 1) % datasets.length);
  }

  // Run packing algo, position items inside bin
  async function pack_bin() {
    // TODO: Implement UI for add/edit/remove items...
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
        const newFreeSpaces: FreeSpace[] = parsedJSON.free_spaces;

        console.log(newItems);

        onBinPacked(newBin);
        onItemsPacked(newItems);
        onFreeSpacesPacked(newFreeSpaces);
    }
  }

  // TEMP: Add a 1x1x1 cube to testSet
  function addCubeToTestSet() {
    const newId = testSet.items.length;
    const newCube: Item = {
      shape_id: newId,
      name: "cube",
      x: 0,
      y: 0,
      z: 0,
      width: 1,
      height: 1,
      depth: 1,
    };
    setTestSet({
      ...testSet,
      items: [...testSet.items, newCube]
    });
  }

  const binDescription = `(${bin.width}×${bin.height}×${bin.depth})`;
  return (
    <>
    <span>
      <h2>{datasets[currentDatasetIndex].name} {binDescription}</h2>
      <form
          className="row"
          onSubmit={(e) => {
            e.preventDefault();
            pack_bin();
          }}
        >
          {/* <button type="submit">Pack</button> */}
          <button type="button" onClick={cycleDataset}>Next Dataset</button>
          {currentDatasetIndex === 0 && <button type="button" onClick={addCubeToTestSet}>Add Cube</button> }
      </form>
    </span>
    </>
  );

}

export default PackerControls;

// Temp test data...
const Cubes = {
  "bin": {
    "width": 7,
    "height": 3,
    "depth": 6
  },
  "items": [
    {
      "shape_id": 0,
      "name": "cube1",
      "x": 0,
      "y": 0,
      "z": 0,
      "width": 2,
      "height": 2,
      "depth": 2,
    },
    {
      "shape_id": 1,
      "name": "cube1",
      "x": 0,
      "y": 0,
      "z": 0,
      "width": 2,
      "height": 2,
      "depth": 2,
    },
    {
      "shape_id": 2,
      "name": "cube1",
      "x": 0,
      "y": 0,
      "z": 0,
      "width": 2,
      "height": 2,
      "depth": 2,
    },
    {
      "shape_id": 3,
      "name": "cube1",
      "x": 0,
      "y": 0,
      "z": 0,
      "width": 2,
      "height": 2,
      "depth": 2,
    },
    {
      "shape_id": 4,
      "name": "cube1",
      "x": 0,
      "y": 0,
      "z": 0,
      "width": 2,
      "height": 2,
      "depth": 2,
    },
    {
      "shape_id": 5,
      "name": "cube1",
      "x": 0,
      "y": 0,
      "z": 0,
      "width": 2,
      "height": 2,
      "depth": 2,
    },
    {
      "shape_id": 6,
      "name": "cube1",
      "x": 0,
      "y": 0,
      "z": 0,
      "width": 2,
      "height": 2,
      "depth": 2,
    },
    {
      "shape_id": 7,
      "name": "cube1",
      "x": 0,
      "y": 0,
      "z": 0,
      "width": 2,
      "height": 2,
      "depth": 2,
    },
      {
      "shape_id": 8,
      "name": "cube1",
      "x": 0,
      "y": 0,
      "z": 0,
      "width": 2,
      "height": 2,
      "depth": 2,
    },
  ],
  "unpacked_items": []};
    

const SimpleData = {
  "bin": {
    "width": 3.0,
    "height": 5.0,
    "depth": 3.0
  },
  "items": [
    {
      "shape_id": 0,
      "name": "example1",
      "x": 0,
      "y": 0,
      "z": 0,
      "width": 2.0,
      "height": 1.0,
      "depth": 3.0,
    },
    {
      "shape_id": 1,
      "name": "example1",
      "x": 0,
      "y": 0,
      "z": 0,
      "width": 1.0,
      "height": 3.0,
      "depth": 2.0,
    },
    {
      "shape_id": 2,
      "name": "example1",
      "x": 0,
      "y": 0,
      "z": 0,
      "width": 1.0,
      "height": 1.0,
      "depth": 5.0,
    },
    {
      "shape_id": 3,
      "name": "example1",
      "x": 0,
      "y": 0,
      "z": 0,
      "width": 1.0,
      "height": 1.0,
      "depth": 4.0,
    },
  ],
  "unpacked_items": []
};


const TestData = {
  "bin": {
    "width": 3,
    "height": 2,
    "depth": 2
  },
  "items": [
    {
      "shape_id": 0,
      "name": "example1",
      "x": 0,
      "y": 0,
      "z": 0,
      "width": 1,
      "height": 0.5,
      "depth": 1,
    },
    {
      "shape_id": 1,
      "name": "example1",
      "x": 0,
      "y": 0,
      "z": 0,
      "width": 2,
      "height": 1,
      "depth": 1,
    },
    {
      "shape_id": 2,
      "name": "example1",
      "x": 0,
      "y": 0,
      "z": 0,
      "width": 1,
      "height": 0.4,
      "depth": 1,
    },
    {
      "shape_id": 3,
      "name": "example1",
      "x": 0,
      "y": 0,
      "z": 0,
      "width": 1,
      "height": 1,
      "depth": 1,
    },
    {
      "shape_id": 4,
      "name": "example1",
      "x": 0,
      "y": 0,
      "z": 0,
      "width": 3,
      "height": 0.2,
      "depth": 1,
    },
  ],
  "unpacked_items": []
};


const HLJData = {
  "bin": {
    "width": 48.1,
    "height": 35.5,
    "depth": 33.6
  },
  "items": [
    {
      "shape_id": 0,
      "name": "liger panzer",
      "x": 0,
      "y": 0,
      "z": 0,
      "width": 40,
      "height": 13.7,
      "depth": 33,
    },
    {
      "shape_id": 1,
      "name": "arhan",
      "x": 0,
      "y": 0,
      "z": 0,
      "width": 30,
      "height": 11,
      "depth": 19,
    },
    // {
    //   "shape_id": 2,
    //   "name": "new arhan",
    //   "x": 0,
    //   "y": 0,
    //   "z": 0,
    //   "width": 30,
    //   "height": 12,
    //   "depth": 21,
    // },
    {
      "shape_id": 3,
      "name": "gqux",
      "x": 0,
      "y": 0,
      "z": 0,
      "width": 30,
      "height": 7,
      "depth": 19,
    },
    {
      "shape_id": 4,
      "name": "VF-25F",
      "x": 0,
      "y": 0,
      "z": 0,
      "width": 31.5,
      "height": 20.5,
      "depth": 11,
    },
    {
      "shape_id": 6,
      "name": "mk-ii aeug",
      "x": 0,
      "y": 0,
      "z": 0,
      "width": 29.8,
      "height": 18.8,
      "depth": 6.7,
    },
  ],
  "unpacked_items": []
};

const HLJData2 = {
  "bin": {
    "width": 48,
    "height": 35,
    "depth": 33
  },
  "items": [
    {
      "shape_id": 0,
      "name": "ao kiriyama liger",
      "x": 0,
      "y": 0,
      "z": 0,
      "width": 31,
      "height": 19.5,
      "depth": 12,
    },
    {
      "shape_id": 1,
      "name": "ao kiriyama liger",
      "x": 0,
      "y": 0,
      "z": 0,
      "width": 31,
      "height": 19.5,
      "depth": 12,
    },
    {
      "shape_id": 2,
      "name": "ao kiriyama liger",
      "x": 0,
      "y": 0,
      "z": 0,
      "width": 31,
      "height": 19.5,
      "depth": 12,
    },
    {
      "shape_id": 3,
      "name": "ao kiriyama liger",
      "x": 0,
      "y": 0,
      "z": 0,
      "width": 31,
      "height": 19.5,
      "depth": 12,
    },
    {
      "shape_id": 4,
      "name": "ao kiriyama liger",
      "x": 0,
      "y": 0,
      "z": 0,
      "width": 31,
      "height": 19.5,
      "depth": 12,
    },
    {
      "shape_id": 5,
      "name": "ao kiriyama liger",
      "x": 0,
      "y": 0,
      "z": 0,
      "width": 31,
      "height": 19.5,
      "depth": 12,
    },
  ]
  ,"unpacked_items": []
};