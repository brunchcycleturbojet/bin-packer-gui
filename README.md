# Bin packer GUI (placeholder name)

![](/public/prototype.png)

**STATUS**: Very WIP! A lot of unresolved TODOs and comments leftover. But the algo seems to be generally working for small cases.
**CURRENT FOCUS**: Cleanup warnings, todos

Lightweight desktop app implementation of the bin packing problem, in 3D.
Made as a vehicle for learning and messing around with Three.js and Rust!

Requirements: <todo: define desktop browser version and types>


## Dev environment
To run: ```npm run tauri dev```


## Todo list

Performance:
- [ ] Swappable heuristics, may help results be more reliable
- [ ] Multi-bin packing 
- [ ] Render items incrementally (large quantity of items packs faster than it renders!)
- [ ] Use instanced meshes for identical items, to reduce draw calls on render
- [ ] Hardware acceleration (Currently CPU only)

Visual/UX:
- [ ] Basic UX (wait for packing on non-ui thread, control inputs display, add/edit/remove item controls)
- [ ] Save/load bin states
- [ ] Colour sets for items (e.g based on size, user defined, random)
- [ ] 3D manipulation in viewer (hide on click, temp removal)
- [ ] Metric/Imperial conversions


## Acknowledgements
Inspired by https://github.com/modulitos/bin_packer_3d/tree/master, which in turn was based on the Shotput implementation: https://medium.com/the-chain/solving-the-box-selection-algorithm-8695df087a4