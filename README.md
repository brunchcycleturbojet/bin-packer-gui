# Bin packer GUI (placeholder name)

<todo: add images>
![](/public/prototype.png)

Lightweight desktop app implementation of the bin packing problem, in 3D.
Made as a vehicle for playing around with Three.js and Rust!

Requirements: <todo: define desktop browser version and types>


## Dev environment
To run: ```npm run tauri dev```


## Planned features

Algo optimisations:
- [ ] Swappable heuristics, may help results be more reliable
- [ ] Multi-bin packing 

UX:
- [ ] Add/remove item
- [ ] Save/load bin states
- [ ] Control inputs display
- [ ] Colour sets for items (e.g based on size, user defined, random)
- [ ] 3D manipulation in viewer (hide on click, temp remove)
- [ ] Metric/Imperial conversions


## Acknowledgements
Inspired by https://github.com/modulitos/bin_packer_3d/tree/master, which in turn was based on the Shotput implementation: https://medium.com/the-chain/solving-the-box-selection-algorithm-8695df087a4