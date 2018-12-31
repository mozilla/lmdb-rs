extern crate pkg_config;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    let mut lmdb: PathBuf = PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap());
    lmdb.push("lmdb");
    lmdb.push("libraries");
    lmdb.push("liblmdb");

    if !pkg_config::find_library("liblmdb").is_ok() {
        let target = env::var("TARGET").expect("No TARGET found");
        let mut build = cc::Build::new();
        if target.contains("android") {
            build.define("ANDROID", "1");
        }
        if target.contains("windows") {
            // LMDB on Windows has an issue with ´off_t´ being defined as ´long´ 32-bit signed
            // which caused a max size of the database of 2 GB which we work 
            // around by redefining ´off_t´ to 64-bit
            // 
            // was discussed here and allgedly fixed in January 2016 but fix doesn't work as 
            // Windows doesn't support _FILE_OFFSET_BITS
            // https://www.openldap.org/lists/openldap-bugs/201605/msg00015.htm
            // https://github.com/LMDB/lmdb/commit/20dec1f69bf4860202c764ce92b1fbbe3d11a065
            build.define("_OFF_T_DEFINED", "1");
            build.define("off_t", "__int64");
            build.define("_off_t", "__int64");
        }

        build
            .file(lmdb.join("mdb.c"))
            .file(lmdb.join("midl.c"))
            // https://github.com/LMDB/lmdb/blob/LMDB_0.9.21/libraries/liblmdb/Makefile#L25
            .opt_level(2)
            .compile("liblmdb.a")
    }
}
