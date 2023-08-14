extern crate cc;

fn main() {
    // println!("cargo:rustc-link-search=all=src");      // works like "rustc -L src ..."
    // println!("cargo:rustc-link-lib=dylib=doubler.o"); // works like "rustc -l doubler.o"
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/native.c");
    println!("cargo:rerun-if-changed=src/native.h");
    println!("cargo:rustc-link-lib=semaphore");
    cc::Build::new()
        .cpp(false)
        .file("src/native.c")
        .flag("-std=gnu99")
        // .cpp_link_stdlib("c++")
        // .cpp_set_stdlib("c++")
        .compile("libsemaphore.a");
}