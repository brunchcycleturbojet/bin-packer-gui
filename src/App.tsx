import "./style/App.css";

import Bin3DView from "./3DView/Bin3DView";
import PackerTable from "./PackerParams";
import PackerResult from "./PackerResult";
import { useAppStore, TabState } from "./store";

function App() {
  const { activeTab, setActiveTab } = useAppStore();

  return (
    <main className="container">
      <div className="row" id="3DContainer">
        <Bin3DView />
      </div>

      <div className="tabs">
        <button 
          className={`tab-button ${activeTab === TabState.Params ? "active" : ""}`}
          onClick={() => setActiveTab(TabState.Params)}
        >
          Parameters
        </button>
        <button 
          className={`tab-button ${activeTab === TabState.Result ? "active" : ""}`}
          onClick={() => setActiveTab(TabState.Result)}
        >
          Result
        </button>
      </div>

      <div id="interfaceContainer">
      {activeTab === TabState.Params ? (
        <PackerTable />
      ) : (
        <PackerResult />
      )}
      </div>

    </main>
  );
}

export default App;