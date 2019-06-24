# Stereo Delay Demo

This is the source code that I prepared for a live coding session to build an audio delay effect.

To build and bundle the VST plugin:

```bash
cargo build --release && ./osx_vst_bundler.sh StereoDelay target/release/libstereo_delay.dylib
```

Follow [this instructions](wasm-delay/README.md) to build and run the WebAssembly version.
