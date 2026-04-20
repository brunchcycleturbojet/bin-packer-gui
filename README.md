# Bin packer GUI (placeholder name)

![](/public/prototype.png)

**STATUS**: WIP! Rough UI is in place. The algo seems to be generally working for smaller cases but requires more testing and adjustments to be considered properly working.
**CURRENT FOCUS**: Get the algo working reliably, then move on to styling the presentation with the current feature set.

Lightweight desktop app implementation of the bin packing problem, in 3D. Made as a vehicle for learning and messing around with Three.js and Rust!

Requirements: <todo: define desktop browser version and types>


## Dev environment
To run: ```npm run tauri dev```


## Todo list

Performance:
- [ ] Render items incrementally (large quantity of items packs faster than it renders!)
- [x] Use instanced meshes for identical items, to reduce draw calls on render
- [ ] Hardware acceleration (Currently CPU only)

Sorting:
- [ ] Resolve cases of overlapping items in large (1000+) item sets
- [ ] Swappable heuristics, may help results be more reliable
- [ ] Multi-bin packing 

Visual/UX:
- [x] Add/edit/remove item controls
- [x] Save/load bin states
- [ ] Report packed/unpacked in table format
- [ ] Save pack results to temp file 
- [ ] Wait for packing on non-ui thread
- [ ] Control inputs guide
- [ ] Colour sets for items (e.g based on size, user defined, random)
- [ ] 3D manipulation in viewer (hide on click, temp removal)
- [ ] Metric/Imperial conversions


## Acknowledgements
Inspired by https://github.com/modulitos/bin_packer_3d/tree/master, which in turn was based on the Shotput implementation: https://medium.com/the-chain/solving-the-box-selection-algorithm-8695df087a4