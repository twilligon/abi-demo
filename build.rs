fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();

    println!("cargo:rustc-link-search=native={}/../../../deps", out_dir);
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}/../../../deps", out_dir);
}
