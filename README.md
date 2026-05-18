# Bin packer GUI (placeholder name)

![](/public/prototype.png)

#### **STATUS**: Barebones UI. The algo is working and often finds optimal or close to optimal solutions, with good performance.

#### **CURRENT FOCUS**: Continue to refine algo logic, and start styling the presentation with the current feature set.

Lightweight desktop app implementation of the bin packing problem, in 3D. Made as a vehicle for learning and messing around with Three.js and Rust!

Requirements: <todo: define desktop browser version and types>


## Dev environment
To run: ```npm run tauri dev```


## Todo list

Performance:
- [ ] Render items incrementally (larger quantities of items packs faster than it renders!)
- [ ] Multi-threading: the algo divides the box into distinct spaces, which could be processed concurrently
- [x] Use instanced meshes for identical items, to reduce draw calls on render
- [ ] Hardware acceleration (Currently CPU only)

Sorting:
- [ ] Consider genetic approach, with targeted mutations, and re-pack to try and achieve maximum bin usage
- [ ] Configurable heuristics (e.g minimise bin size, minimise X/Y/Z use, consider stability)
- [ ] Multi-bin packing 

Visual/UX:
- [ ] Instanced results report in tabs
- [ ] Export/Import results to CSV 
- [ ] Wait for packing on non-ui thread
- [ ] Control inputs guide
- [ ] Colour sets for items (e.g based on size, user defined, random)
- [ ] 3D manipulation in viewer (hide on click, temp removal)
- [ ] Metric/Imperial conversions


## Acknowledgements
Inspired by https://github.com/modulitos/bin_packer_3d/tree/master, which in turn was based on the Shotput implementation: https://medium.com/the-chain/solving-the-box-selection-algorithm-8695df087a4