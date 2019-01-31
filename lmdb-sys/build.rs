extern crate pkg_config;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    let mut lmdb: PathBuf = PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap());
    lmdb.push("lmdb");
    lmdb.push("libraries");
    lmdb.push("liblmdb");

    let mut undefine_have_memalign_h: PathBuf = PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap());
    undefine_have_memalign_h.push("undefine-have-memalign.h");

    if !pkg_config::find_library("liblmdb").is_ok() {
        let target = env::var("TARGET").expect("No TARGET found");
        let mut build = cc::Build::new();
        if target.contains("android") {
            build.define("ANDROID", "1");
        }
        build
            .file(lmdb.join("mdb.c"))
            .file(lmdb.join("midl.c"))
            // https://github.com/LMDB/lmdb/blob/LMDB_0.9.21/libraries/liblmdb/Makefile#L25
            .opt_level(2);

        // Undefine HAVE_MEMALIGN via -include preprocessor flag to avoid
        // implicit function declaration when mdb.c uses memalign() without
        // including malloc.h (https://github.com/mozilla/lmdb-rs/pull/28).
        //
        // It'd presumably be better to test if is_flag_supported("-include"),
        // but that doesn't seem to work, perhaps because the -include flag
        // is a preprocessor flag rather than a compiler flag?
        if !build.get_compiler().is_like_msvc() {
            build.flag("-include").flag(undefine_have_memalign_h.to_str().unwrap());
        }

        build.compile("liblmdb.a");
    }
}
