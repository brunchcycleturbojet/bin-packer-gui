import { useState } from "react";
import "./style/App.css";

import {Bin, Item, FreeSpace} from "./BinData";
import Bin3DView from "./3DView/Bin3DView";
import PackerTable from "./PackerTable";

const initialBin: Bin = new Bin(4, 4, 4);
const initialItems: Item[] = [];
const initialFreeSpaces: FreeSpace[] = [];

function App() {
  const [bin, setBin] = useState(initialBin);
  const [items, setItems] = useState(initialItems);
  const [freeSpaces, setFreeSpaces] = useState(initialFreeSpaces);

  function updateBin(newBin: Bin) {
    setBin(newBin);
  }

  function updateItems(newItems: Item[]) {
    setItems([...newItems]);
  }

  function updateFreeSpaces(newFreeSpaces: FreeSpace[]) {
    setFreeSpaces([...newFreeSpaces]);
  }

  return (
    <main className="container">
      <div className="row" id="3DContainer">
        <Bin3DView bin={bin} items={items} freeSpaces={freeSpaces}/>
      </div>

      <PackerTable bin={bin} items={items} onItemsPacked={updateItems} onBinPacked={updateBin} onFreeSpacesPacked={updateFreeSpaces}/>
    </main>
  );
}

export default App;