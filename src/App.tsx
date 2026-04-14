import { useState } from "react";
import "./style/App.css";

import {Bin, Item, FreeSpace} from "./BinData";
import Bin3DView from "./3DView/Bin3DView";
import PackerTable from "./PackerTable";
import PackerControls from "./PackerControls";

const initialBin: Bin = new Bin("Untitled bin", 4, 4, 4);
const initialItems: Item[] = [];
const initialFreeSpaces: FreeSpace[] = [];

function App() {
  const [bin, setBin] = useState(initialBin);
  const [items, setItems] = useState(initialItems);
  const [freeSpaces, setFreeSpaces] = useState(initialFreeSpaces);
  const [showFreeSpaces, setShowFreeSpaces] = useState(false);

  function updateBin(newBin: Bin) {
    setBin(newBin);
  }

  function updateItems(newItems: Item[]) {
    // Trigger re-render by completely replacing current array with the new one.
    setItems([...newItems]);
  }

  function updateFreeSpaces(newFreeSpaces: FreeSpace[]) {
    setFreeSpaces([...newFreeSpaces]);
  }

  function toggleFreeSpaces() {
    setShowFreeSpaces(prev => !prev);
  }

  return (
    <main className="container">
      <div className="row" id="3DContainer">
        <Bin3DView bin={bin} items={items} freeSpaces={freeSpaces} showFreeSpaces={showFreeSpaces}/>
      </div>

      <PackerControls bin={bin} items={items} onItemsPacked={updateItems} onBinPacked={updateBin} onFreeSpacesPacked={updateFreeSpaces} onToggleFreeSpaces={toggleFreeSpaces} />

      <PackerTable bin={bin} items={items} />
    </main>
  );
}

export default App;