fn main() {
    let target = std::env::var("TARGET").unwrap();
    if target == "x86_64-unknown-linux-gnu" {
        println!("cargo:rustc-link-lib=vulkan");
    }
}
