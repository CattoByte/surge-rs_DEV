use std::{collections::HashSet, env, path::PathBuf};

fn main() {
    let dst = cmake::Config::new("sbmod/surge")
        .define("SURGE_SKIP_JUCE_FOR_RACK", "ON")
        .define("SURGE_SKIP_VST3", "ON")
        .define("SURGE_SKIP_ALSA", "ON")
        .define("SURGE_SKIP_STANDALONE", "ON")
        .define("SURGE_SKIP_LUA", "ON")
        .define("CMAKE_EXPORT_COMPILE_COMMANDS", "ON")
        .build();
    // i know.
    println!("cargo:rustc-link-search=native={}", dst.join("build/src/common").display());
    println!("cargo:rustc-link-search=native={}", dst.join("build/src/lua").display());
    println!("cargo:rustc-link-search=native={}", dst.join("build/libs/zstd/build/cmake/lib").display());
    println!("cargo:rustc-link-search=native={}", dst.join("build/libs/sqlite-3.23.3").display());
    println!("cargo:rustc-link-search=native={}", dst.join("build/libs/oddsound-mts").display());
    println!("cargo:rustc-link-search=native={}", dst.join("build/libs/fmt").display());
    println!("cargo:rustc-link-search=native={}", dst.join("build/libs/pffft").display());
    println!("cargo:rustc-link-search=native={}", dst.join("build/libs/eurorack").display());
    println!("cargo:rustc-link-search=native={}", dst.join("build/libs/binn").display());
    println!("cargo:rustc-link-search=native={}", dst.join("build/libs/airwindows").display());
    println!("cargo:rustc-link-search=native={}", dst.join("build/libs/sst/sst-plugininfra").display());
    println!("cargo:rustc-link-search=native={}", dst.join("build/libs/sst/sst-plugininfra/libs/strnatcmp").display());
    println!("cargo:rustc-link-search=native={}", dst.join("build/libs/sst/sst-plugininfra/libs/tinyxml").display());
    println!("cargo:rustc-link-lib=static=surge-lua-src");
    println!("cargo:rustc-link-lib=static=surge-common");
    println!("cargo:rustc-link-lib=static=zstd");
    println!("cargo:rustc-link-lib=static=sqlite");
    println!("cargo:rustc-link-lib=static=oddsound-mts");
    println!("cargo:rustc-link-lib=static=fmtd");
    println!("cargo:rustc-link-lib=static=pffft");
    println!("cargo:rustc-link-lib=static=eurorack");
    println!("cargo:rustc-link-lib=static=binn");
    println!("cargo:rustc-link-lib=static=airwindows");
    println!("cargo:rustc-link-lib=static=sst-plugininfra");
    println!("cargo:rustc-link-lib=static=strnatcmp");
    println!("cargo:rustc-link-lib=static=tinyxml");

/*./target/debug/build/surge-rs-b9682071e1fd954b/out/libbridge.a
./target/debug/build/surge-rs-b9682071e1fd954b/out/build/src/lua/libsurge-lua-src.a
./target/debug/build/surge-rs-b9682071e1fd954b/out/build/src/common/libsurge-common.a
./target/debug/build/surge-rs-b9682071e1fd954b/out/build/libs/zstd/build/cmake/lib/libzstd.a
./target/debug/build/surge-rs-b9682071e1fd954b/out/build/libs/sqlite-3.23.3/libsqlite.a
./target/debug/build/surge-rs-b9682071e1fd954b/out/build/libs/oddsound-mts/liboddsound-mts.a
./target/debug/build/surge-rs-b9682071e1fd954b/out/build/libs/fmt/libfmtd.a
./target/debug/build/surge-rs-b9682071e1fd954b/out/build/libs/pffft/libpffft.a
./target/debug/build/surge-rs-b9682071e1fd954b/out/build/libs/eurorack/libeurorack.a
./target/debug/build/surge-rs-b9682071e1fd954b/out/build/libs/binn/libbinn.a
./target/debug/build/surge-rs-b9682071e1fd954b/out/build/libs/airwindows/libairwindows.a
./target/debug/build/surge-rs-b9682071e1fd954b/out/build/libs/sst/sst-plugininfra/libsst-plugininfra.a
./target/debug/build/surge-rs-b9682071e1fd954b/out/build/libs/sst/sst-plugininfra/libs/strnatcmp/libstrnatcmp.a
./target/debug/build/surge-rs-b9682071e1fd954b/out/build/libs/sst/sst-plugininfra/libs/tinyxml/libtinyxml.a*/



    println!("cargo:rerun-if-changed=cpp/bridge.cpp");
    println!("cargo:rerun-if-changed=cpp/bridge.h");
    let mut bbuild = cc::Build::new();
    bbuild
        .warnings(false)
        .cpp(true)
        .std("c++20")
	.flag("-fno-char8_t")          // read ahead. this has to go here too...
        .file("cpp/bridge.cpp");

    let comcom = dst.join("build/compile_commands.json");   // "compile commands". comcom.
    let json = std::fs::read_to_string(&comcom).expect("failed to read comcom!");
    let coms: serde_json::Value = serde_json::from_str(&json).expect("failed to parse comcom!");

    let mut bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++20")
	.clang_arg("-fno-char8_t")          // fix for compilation. present in cmake, surely.
        .layout_tests(false)                // fix for unnecessary checks that overflow (good job).
        .opaque_type("std::.*")             // fix for stl type exports (obvious).
        .blocklist_item("fmt::.*")          // fix for formatting lib exports (can't be represented).
        .blocklist_item("FP_INT__.*")       // fix for double definition (math.h likely).
        .blocklist_item("size_type")        // fix for something with a looping equivalent (somehow).
        .blocklist_item("const_pointer")    // fix for multiple definitions (of a basic term).
        .blocklist_item("rep")              // fix for multiple definitions (of whatever that is).
        .blocklist_item("int_type")         // fix for multiple definitions (of a second basic term).
        .blocklist_item("char_type")        // fix for multiple definitions (of a third basic term).
        .blocklist_item("iterator")         // fix for multiple definitions (of a complex term).
        .blocklist_item("FE_.*")            // fix for various double definitions (FE?).
        .blocklist_item("FP_.*")            // fix for various double definitions (FE counterpart?).
        .blocklist_item("__gnu_.*")         // fix for proprietary data (somewhat).
        .allowlist_item("Surge.*")          // fix for everything else (the nuclear option).
        .allowlist_item(".*idFor.*")        // fix for functions i need (unexported).
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));

    // get and use all the include paths from the configure.
    let mut unique = HashSet::new();
    for entry in coms.as_array().unwrap() {
        if let Some(clist) = entry.get("command") {
            shell_words::split(clist.as_str().unwrap())
                .unwrap()
                .into_iter()
                .filter(|x| x.starts_with("-I") || x.starts_with("-DSURGE"))
                .for_each(|x| { unique.insert(x); })
        }
    }

    // not sorting *will* crash the build.
    let mut tempvec: Vec<_> = unique.into_iter().collect();
    tempvec.sort();
    for flag in tempvec {
        eprintln!("new flag: {}", flag);
        bbuild.flag(&flag);
        bindings = bindings.clone().clang_arg(&flag);
    }

    let out = bbuild.try_compile("bridge");
    if let Err(e) = out { panic!("bridge burnt down while building.\n\n{}", e); }
    println!("cargo:rustc-link-lib=static=bridge");

    let bindings = bindings.generate().expect("unable to generate surge bindings");
    let storehere = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(storehere.join("bindings.rs"))
        .expect("couldn't write bindings.");
}
