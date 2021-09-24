## Notes
* Before running testing in `ffi.rs`, ensure `lib/libsnappy.a` exists.
* If not exists, download the [release](https://github.com/google/snappy/releases/), go to the download directory and run the following commands to build `libsnappy.a` (for `mingw32-make` and `rustup toolchain x86_64-pc-windows-gnu`).
```bash
mkdir build && cd build
cmake -G 'MinGW Makefiles' -DSNAPPY_BUILD_TESTS=OFF -DSNAPPY_BUILD_BENCHMARKS=OFF ../
```