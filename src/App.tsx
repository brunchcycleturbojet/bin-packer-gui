import { useState } from "react";
import "./style/App.css";

import {Bin, Item} from "./BinData";
import Bin3DView from "./3DView/Bin3DView";
import PackerTable from "./PackerTable";
import PackerControls from "./PackerControls";

const initialBin: Bin = new Bin("Untitled bin", 4, 4, 4);
const initialItems: Item[] = [];

function App() {
  const [bin, setBin] = useState(initialBin);
  const [items, setItems] = useState(initialItems);

  function updateBin(newBin: Bin) {
    setBin(newBin);
  }

  function updateItems(newItems: Item[]) {
    // Trigger re-render by completely replacing current array with the new one.
    setItems([...newItems]);
  }

  return (
    <main className="container">
      <div className="row" id="3DContainer">
        <Bin3DView bin={bin} items={items}/>
      </div>

      <PackerControls bin={bin} items={items} onItemsPacked={updateItems} onBinPacked={updateBin} />

      {/* <PackerTable bin={bin} items={items} /> */}
    </main>
  );
}

export default App;