pub fn main() {
    println!("cargo:rerun-if-changed=src/lab.ld");
    println!("cargo:rerun-if-changed=src/start.S");
}
