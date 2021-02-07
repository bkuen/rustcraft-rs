# Rustcraft
A Minecraft clone written in Rust using OpenGL
At the moment only one 8x8 chunk will be displayed
using greedy meshing techniques.

Rendering a single chunk
![Chunk](https://image.cod3.eu/280-pGPFjYU9)
![Greedy Meshing](https://image.cod3.eu/281-9YIqaLNL)

Now, more chunks are being rendered while moving the camera. At the moment, I don't care about unloading chunks and freeing memory if a chunk is out of the render distance.
![ChunkRendering](https://image.cod3.eu/305-I3vVXC3s)
![ChunkRenderingMesh](https://image.cod3.eu/306-CXPducC2)

## Installation
```bash
git clone https://github.com/bkuen/rustcraft-rs.git
cd rustcraft-rs
cargo run --release
```

## License
The code of this repository is licensed under GNU GPLv3 ([LICENSE](./LICENSE) or https://opensource.org/licenses/GPL-3.0)
