fn main() {
    let out_dir = "/home/shengael/Documents/__esgi/rust/ppma";
    println!("cargo:rustc-link-search=native={}", out_dir);
}