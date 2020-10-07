1. Install msys2 (see: https://www.msys2.org/).
2. Inside msys2, install rust:
   ```
   pacman -S --needed mingw-w64-x86_64-toolchain base-devel git vim mingw-w64-x86_64-cmake mingw-w64-x86_64-rust
   ```
3. Clone the repository.
4. Set some variables:
   ```
   export PATH=/mingw64/bin:$PATH
   ```
5. Build:
   ```
   cargo build --release
   ```
6. Create a folder containg the folders ```icons```, ```locale``` and ```sounds```,
   the files ```LICENSE``` and ```yabinero.exe```.
