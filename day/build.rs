use std::path::Path;

fn main() {
    if Path::new("input.txt").exists() {
        println!("cargo::rustc-cfg=input_txt");
    }
    if Path::new("part1.txt").exists() {
        println!("cargo::rustc-cfg=part1_txt");
    }
    if Path::new("part2.txt").exists() {
        println!("cargo::rustc-cfg=part2_txt");
    }
    println!("cargo::rerun-if-changed=input.txt");
    println!("cargo::rerun-if-changed=part1.txt");
    println!("cargo::rerun-if-changed=part2.txt");
}
