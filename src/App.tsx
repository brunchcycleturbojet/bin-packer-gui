import "./style/App.css";

import Bin3DView from "./3DView/Bin3DView";
import PackerTable from "./PackerTable";
import PackerResult from "./PackerResult";
import { useAppStore, TabState } from "./store";

function App() {
  const { bin, items, freeSpaces, activeTab, setActiveTab } = useAppStore();

  return (
    <main className="container">
      <div className="row" id="3DContainer">
        <Bin3DView />
      </div>

      <div className="tabs">
        <button 
          className={`tab-button ${activeTab === TabState.Table ? "active" : ""}`}
          onClick={() => setActiveTab(TabState.Table)}
        >
          Packer Table
        </button>
        <button 
          className={`tab-button ${activeTab === TabState.Result ? "active" : ""}`}
          onClick={() => setActiveTab(TabState.Result)}
        >
          Result
        </button>
      </div>

      <div id="interfaceContainer">
      {activeTab === TabState.Table ? (
        <PackerTable />
      ) : (
        <PackerResult />
      )}
      </div>

    </main>
  );
}

export default App;