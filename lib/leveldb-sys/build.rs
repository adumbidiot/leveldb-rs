use std::path::PathBuf;

fn build_snappy() -> PathBuf {
    let snappy_dir = cmake::Config::new("snappy-1.1.8")
        .uses_cxx11()
        .define("SNAPPY_BUILD_TESTS", "OFF")
        .build();
    println!("cargo:rustc-link-search=native={}", snappy_dir.display());
    println!(
        "cargo:rustc-link-search=native={}/lib",
        snappy_dir.display()
    );
    println!("cargo:rustc-link-lib=static=snappy");
    snappy_dir
}

fn main() {
    let snappy_dir = build_snappy();
    let snappy_lib_dir = format!("{}/lib", snappy_dir.display());

    let install_dir = cmake::Config::new("leveldb-1.22")
        .uses_cxx11()
        .define(
            "CMAKE_CXX_FLAGS",
            &format!("-I{}/include", snappy_dir.display()),
        )
        .define(
            "CMAKE_EXE_LINKER_FLAGS",
            &format!("-libpath:{dir} -L{dir}", dir = snappy_lib_dir),
        ) // Why is every build system a burning trashfire? Waiting for (https://github.com/google/leveldb/pull/686)
        .define("LEVELDB_BUILD_TESTS", "OFF")
        .define("LEVELDB_BUILD_BENCHMARKS", "OFF")
        .build();

    println!("cargo:rustc-link-search=native={}", install_dir.display());
    println!(
        "cargo:rustc-link-search=native={}/lib",
        install_dir.display()
    );
    println!("cargo:rustc-link-lib=static=leveldb");

    let bindings = bindgen::builder()
        .header("wrapper.h")
        .clang_arg(&format!("-I{}/include", install_dir.display()))
        .whitelist_type("leveldb_.*")
        .whitelist_function("leveldb_.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
