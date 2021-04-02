fn main() {
    cc::Build::new()
        .file("c/MurmurHash3.c")
        .include("c")
        .compile("MurmurHash3");
}
