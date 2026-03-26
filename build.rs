fn main() {
    // Enable WASM JS support for getrandom
    #[cfg(target_arch = "wasm32")]
    {
        println!("cargo:rustc-env=CARGO_CFG_TARGET_FEATURE=+simd128");
    }
}
