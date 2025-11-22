fn main() {
    println!("cargo:rustc-link-arg=-fuse-ld=lld");
    println!(
        "cargo:rustc-link-arg=-Wl,--icf={}",
        std::env::var("ICF").as_deref().unwrap_or("all")
    );
}
