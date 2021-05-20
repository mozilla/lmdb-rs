use bindgen::callbacks::IntKind;
use bindgen::callbacks::ParseCallbacks;
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
struct Callbacks;

impl ParseCallbacks for Callbacks {
    fn int_macro(&self, name: &str, _value: i64) -> Option<IntKind> {
        match name {
            "MDB_SUCCESS"
            | "MDB_KEYEXIST"
            | "MDB_NOTFOUND"
            | "MDB_PAGE_NOTFOUND"
            | "MDB_CORRUPTED"
            | "MDB_PANIC"
            | "MDB_VERSION_MISMATCH"
            | "MDB_INVALID"
            | "MDB_MAP_FULL"
            | "MDB_DBS_FULL"
            | "MDB_READERS_FULL"
            | "MDB_TLS_FULL"
            | "MDB_TXN_FULL"
            | "MDB_CURSOR_FULL"
            | "MDB_PAGE_FULL"
            | "MDB_MAP_RESIZED"
            | "MDB_INCOMPATIBLE"
            | "MDB_BAD_RSLOT"
            | "MDB_BAD_TXN"
            | "MDB_BAD_VALSIZE"
            | "MDB_BAD_DBI"
            | "MDB_LAST_ERRCODE" => Some(IntKind::Int),
            "MDB_SIZE_MAX" | "MDB_PROBLEM" => Some(IntKind::U64),
            _ => Some(IntKind::UInt),
        }
    }
}

pub fn generate() {
    let mut lmdb;
    match pkg_config::probe_library("lmdb") {
        Ok(mut library) => {
            lmdb = library.include_paths.pop().unwrap();
        }
        Err(_) => {
            lmdb = PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap());
            lmdb.push("lmdb");
            lmdb.push("libraries");
            lmdb.push("liblmdb");
        }
    }

    let src_bindings_path = PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("src")
        .join("bindings.rs");
    let out_bindings_path =
        PathBuf::from(env::var("OUT_DIR").unwrap()).join("lmdb-sys-bindings.rs");

    let bindings = bindgen::Builder::default()
        .header(lmdb.join("lmdb.h").to_string_lossy())
        .whitelist_var("^(MDB|mdb)_.*")
        .whitelist_type("^(MDB|mdb)_.*")
        .whitelist_function("^(MDB|mdb)_.*")
        .size_t_is_usize(true)
        .ctypes_prefix("::libc")
        .blacklist_item("mode_t")
        .blacklist_item("mdb_mode_t")
        .blacklist_item("mdb_filehandle_t")
        .blacklist_item("^__.*")
        .parse_callbacks(Box::new(Callbacks {}))
        .layout_tests(false)
        .prepend_enum_name(false)
        .rustfmt_bindings(true)
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(&out_bindings_path)
        .expect("Couldn't write bindings!");

    let previous_contents = match fs::read(&src_bindings_path) {
        Ok(s) => s,
        Err(_) => vec![],
    };

    let new_contents = match fs::read(&out_bindings_path) {
        Ok(s) => s,
        Err(_) => vec![],
    };

    if previous_contents != new_contents {
        // The bindings.rs were changed, update the original contents of src/bindings.rs
        fs::copy(out_bindings_path, src_bindings_path).expect("Unable to write new bindings.");
    };
}
