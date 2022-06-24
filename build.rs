fn main() {
    // Linker workaround for openssl with musl
    println!("cargo:rustc-link-arg=-lc");
}