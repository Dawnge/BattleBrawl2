# BattleBrawl2
2D 1v1 Arena Battle Brawler 2 Game written in Rust called BattleBrawl2

## Design:

BattleBrawl2 is a offline 1vs1 Brawler Game where you punch your enemy to DEATH!!!  
You will fight an AI or another Player in an Arena. The goal is to defeat the opponent by reducing his HP to zero.  
Your character can move left, right and jump. He interacts with his opponent by punching, which lowers his HP. 
- [X] Create Checklist
- [ ] Create Game State and Loop
  - [ ] processInput();
  - [ ] update();
  - [ ] render();

## Setup BattleBrawl2 for cross compile wasm and windows-gnu on Unbuntu linux 
Install dependencies
```
sudo apt install pkg-config libx11-dev libxi-dev libgl1-mesa-dev libasound2-dev  
```


Setup Windows cross-compile
```
sudo apt install build-essential
sudo apt-get install mingw-w64
rustup target add x86_64-pc-windows-gnu  
rustup toolchain install stable-gnu # idk if this is necessary
rustup toolchain install stable-x86_64-pc-windows-gnu  
```

Setup WASM cross-compile
```
rustup target add wasm32-unknown-unknown
rustup toolchain install wasm32-unknown-unknown
```


To run the WASM thing we use the basic-http-server crate
```
cargo install basic-http-server
# if you encounter problems when installing in WSL, restart pc :D
basic-http-server .
```
