import { create } from 'zustand';
import { Bin, Item, FreeSpace } from './binData';

enum TabState {
  Table = "table",
  Result = "result",
}

interface AppStore {
  bin: Bin;
  items: Item[];
  freeSpaces: FreeSpace[];
  pendingBin: Bin;
  pendingItems: Item[];
  activeTab: TabState;
  lastPackedItems: Item[];
  lastUnpackedItems: Item[];
  
  updateBin: (newBin: Bin) => void;
  updateItems: (newItems: Item[]) => void;
  updateFreeSpaces: (newFreeSpaces: FreeSpace[]) => void;
  updatePendingBin: (newPendingBin: Bin) => void;
  updatePendingItems: (newPendingItems: Item[]) => void;
  setActiveTab: (tab: TabState) => void;
  updateLastPackedItems: (packedItems: Item[]) => void;
  updateLastUnpackedItems: (unpackedItems: Item[]) => void;
}

const initialBin = new Bin(4, 4, 4);

export const useAppStore = create<AppStore>((set) => ({
  bin: initialBin,
  items: [],
  freeSpaces: [],
  pendingBin: initialBin,
  pendingItems: [],
  activeTab: TabState.Table,
  lastPackedItems: [],
  lastUnpackedItems: [],

  updateBin: (newBin) => set({ bin: newBin }),
  updateItems: (newItems) => set({ items: [...newItems] }),
  updateFreeSpaces: (newFreeSpaces) => set({ freeSpaces: [...newFreeSpaces] }),
  updatePendingBin: (newPendingBin) => set({ pendingBin: newPendingBin }),
  updatePendingItems: (newPendingItems) => set({ pendingItems: newPendingItems }),
  setActiveTab: (tab) => set({ activeTab: tab }),
  updateLastPackedItems: (packedItems) => set({ lastPackedItems: [...packedItems] }),
  updateLastUnpackedItems: (unpackedItems) => set({ lastUnpackedItems: [...unpackedItems] }),

}));

export { TabState };
