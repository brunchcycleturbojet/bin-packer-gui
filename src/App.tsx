import "./style/App.css";

import Bin3DView from "./3DView/Bin3DView";
import PackerTable from "./PackerParams";
import PackerResult from "./PackerResult";
import { useAppStore, TabState } from "./store";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "./components/ui/tabs";
import { ThemeProvider } from "./components/theme-provider";
import { ModeToggle } from "./components/mode-toggle";
import { ResizableHandle, ResizablePanel, ResizablePanelGroup } from "#components/ui/resizable";

function AppContent() {
  const { activeTab, setActiveTab } = useAppStore();

  return (
    <main className="flex h-screen w-screen flex-row max-h-dvh max-w-dvw">
      <ResizablePanelGroup orientation="horizontal">
        <ResizablePanel>
          <div className="flex-[2_2] h-full relative min-w-0">
            <Bin3DView />
            <div className="absolute top-16 right-4 z-10">
              <ModeToggle />
            </div>
          </div>
        </ResizablePanel>
        <ResizableHandle withHandle />
        <ResizablePanel minSize={"30vw"}>
          <div className="flex-[1_1] max-h-dvh min-h-0 flex-col min-w-0 ">
            <Tabs
              value={activeTab}
              onValueChange={(value) => setActiveTab(value as TabState)}
              className="w-full"
            >
              <TabsList>
                <TabsTrigger value={TabState.Params}>Parameters</TabsTrigger>
                <TabsTrigger value={TabState.Result}>Result</TabsTrigger>
              </TabsList>

              <TabsContent value={TabState.Params}>
                <PackerTable />
              </TabsContent>

              <TabsContent value={TabState.Result}>
                <PackerResult />
              </TabsContent>
            </Tabs>
          </div>
        </ResizablePanel>
      </ResizablePanelGroup>
    </main>
  );
}

function App() {
  return (
    <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
      <AppContent />
    </ThemeProvider>
  );
}

export default App;