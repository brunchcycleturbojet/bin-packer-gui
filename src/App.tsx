import "./style/App.css";

import Bin3DView from "./3DView/Bin3DView";
import PackerParams from "./components/packer-params";
import PackerResult from "./components/packer-result";
import { useAppStore, TabState } from "./store";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "./components/ui/tabs";
import { ThemeProvider } from "./components/theme-provider";
import { LightDarkToggle } from "./components/light-dark-toggle";
import { ResizableHandle, ResizablePanel, ResizablePanelGroup } from "#components/ui/resizable";
import { TooltipProvider } from "#components/ui/tooltip";
import PackerSummary from "./components/packer-summary";

function AppContent() {
  const { activeTab, setActiveTab, lastPackedItems, lastUnpackedItems } = useAppStore();

  const totalPacked = lastPackedItems.reduce((acc, curr) => {
    return acc + curr.quantity;
  }, 0)
  const totalNotPacked = lastUnpackedItems.reduce((acc, curr) => {
    return acc + curr.quantity;
  }, 0)

  return (
    <main className="flex h-screen w-screen flex-row max-h-dvh max-w-dvw">
      <ResizablePanelGroup orientation="horizontal">
        <ResizablePanel>
          <div className="flex-[2_2] h-full relative min-w-0">
            <Bin3DView />
          </div>
        </ResizablePanel>
        <ResizableHandle withHandle className="border-t-amber-500"/>
        <ResizablePanel minSize={"30vw"}>
          <ResizablePanelGroup orientation="vertical">
            <ResizablePanel>
              <div className="flex-[1_1] max-h-dvh min-h-0 flex-col min-w-0 overflow-hidden">
                <div className="absolute top-1.5 right-1.5 z-10">
                  <LightDarkToggle />
                </div>
                <PackerParams />
              </div>
            </ResizablePanel>
            <ResizableHandle withHandle className="border-t-amber-500"/>
            <ResizablePanel>
              <Tabs                   
                defaultValue={TabState.Packed}
                value={activeTab}
                onValueChange={(value) => setActiveTab(value as TabState)}
                className="w-full">
                  <TabsList className="ml-2 mt-1.5">
                    <TabsTrigger className="text-sm p-4 m-1" value={TabState.Stats}>Summary</TabsTrigger>
                    <TabsTrigger className="text-sm p-4 m-1" value={TabState.Packed}>Packed ({totalPacked})</TabsTrigger>
                    <TabsTrigger className="text-sm p-4 m-1" value={TabState.NotPacked}>Remainder ({totalNotPacked})</TabsTrigger>
                  </TabsList>
                  <TabsContent value={TabState.Stats}>
                    <PackerSummary />
                  </TabsContent>
                  <TabsContent value={TabState.Packed}>
                    <PackerResult items={lastPackedItems}/>
                  </TabsContent>
                  <TabsContent value={TabState.NotPacked}>
                    <PackerResult items={lastUnpackedItems}/>
                  </TabsContent>
              </Tabs>
              <div className="border-t w-full pl-2 pt-1 pb-1 absolute bottom-0 text-sm bg-background text-muted-foreground" > {/* Log (TODO), e.g save reminder*/} Log: Hello!</div>
            </ResizablePanel>
          </ResizablePanelGroup>
        </ResizablePanel>
      </ResizablePanelGroup>
    </main>
  );
}

function App() {
  return (
    <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
      <TooltipProvider>
        <AppContent />
      </TooltipProvider>
    </ThemeProvider>
  );
}

export default App;