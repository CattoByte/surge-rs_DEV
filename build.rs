use std::{collections::HashSet, env, path::{Path, PathBuf}};
use git2::build::CheckoutBuilder;

use {bindgen, serde_json, shell_words, cmake, cc, git2};
// ^ here to easily check if they go unused.

fn pct_callback(c: &mut u32, p: git2::Progress<'_>) -> bool {
    *c += 1;
    if *c == 10 {
        *c = 0;
        let percentage =
            p.received_objects() as f32
            / p.total_objects() as f32
            * 100.0;

        println!(
            "\x1B[APULL...  {:6.2}%  ({: >5}/{: <5}); {} bytes.",
            percentage,
            p.received_objects(),
            p.total_objects(),
            p.received_bytes(),
        );
    }

    true
}

fn chk_callback(p: Option<&Path>, c: usize, t: usize) {
    let mut p = if let Some(path) = p {
        path.to_string_lossy().to_string()
    } else {
        "???".to_string()
    };
    if p.chars().count() > 25 {
        p = "...".to_string() + &p.chars()
            .rev().take(22).collect::<String>().chars()
            .rev().collect::<String>();
    }

    let percentage = c as f32 / t as f32 * 100.0;

    println!(
        "\x1B[ACHECK... {:6.2}%  ({: >5}/{: <5}); @ {: >25}.",
        percentage,
        c,
        t,
        p,
    );
}

fn sm_update_rec(repo: &git2::Repository) {
    let sms = repo.submodules().expect("the surge's insides are devoid of instructions...");
    for mut sm in sms {
        let name = sm.name().unwrap_or("???");
        println!("injecting \"{}\" into the surge.", name);

        sm.init(false).expect("joy initialization failed.");
        let mut counter = 0;
        let mut uopts = git2::SubmoduleUpdateOptions::new();
        let mut fopts = git2::FetchOptions::new();
        let mut callbacks = git2::RemoteCallbacks::new();
        let mut checkout = CheckoutBuilder::new();
        callbacks.transfer_progress(|p| pct_callback(&mut counter, p));
        checkout.progress(|p, c, t| chk_callback(p, c, t));
        fopts.remote_callbacks(callbacks).prune(git2::FetchPrune::On);
        uopts.fetch(fopts).checkout(checkout);

        println!("...");
        sm.update(true, Some(&mut uopts)).expect("failed to introduce into the surge.");
        println!("\x1B[AOK.                                                           ");

        if let Ok(repo) = sm.open() {
            if repo.submodules().unwrap().len() > 0 {
                println!("found extra goodies to insert.");
                sm_update_rec(&repo);
            }
        }
    }
}

fn pull_surge_from_clouds(dst: &Path) {
    if dst.exists() {
        if git2::Repository::open(dst).unwrap().head().is_ok() {
            println!("surge is down from the clouds. no action.");
            return;
        } else {
            println!("surge is down from the clouds, but it came down mangled.");
            assert_eq!(dst.to_str().unwrap(), "sbmod/surge/");    // just as safety.
            std::fs::remove_dir_all(dst).unwrap();
            println!("removed the mangled surge. poor thing.");
        }
    }

    println!("surge is in the sky. pulling surge from the clouds.");
    let mut counter = 0;
    let mut callbacks = git2::RemoteCallbacks::new();
    let mut checkout = CheckoutBuilder::new();
    callbacks.transfer_progress(|p| pct_callback(&mut counter, p));
    checkout.progress(|p, c, t| chk_callback(p, c, t));
        //.sideband_progress(|txt| { println!("{:?}", txt); true})
        //.pack_progress(|_, d2, d3| println!("{}\t{}", d2, d3))


    let mut fopts = git2::FetchOptions::new();
    fopts.depth(1).remote_callbacks(callbacks).prune(git2::FetchPrune::On);

    println!("...");
    git2::build::RepoBuilder::new()
        .fetch_options(fopts)
        .with_checkout(checkout)
        .clone("https://github.com/surge-synthesizer/surge", dst)
        .expect("the sun came up, so we were unable to pull surge from the clouds.");
    println!("\x1B[AOK.                                                           ");

    // sorry for writing this one. m(._.)m
    println!("the pulled surge is stable, but we need to fill its innards with joy.");
    let repo = git2::Repository::open(dst).expect("somehow couldn't crack open the surge.");
    sm_update_rec(&repo);
    println!("surge is ready.");

}

fn main() {
    // TODO: allow custom directory or keep tree mode?
    let (spath, bpath) = if env::var("CARGO_FEATURE_IN_SURGE_TREE").is_ok() {
        println!("feature \"in-surge-tree\" enabled. using parent directories.");
        ("../..".to_string(), "../..".to_string())
    } else {
        println!("feature \"in-surge-tree\" disabled. pulling surge.");
        let sdst = "sbmod/surge".to_string();
        pull_surge_from_clouds(&Path::new(&sdst));
        let bdst = cmake::Config::new(&sdst)
            .define("SURGE_SKIP_JUCE_FOR_RACK", "ON")
            .define("SURGE_SKIP_VST3", "ON")
            .define("SURGE_SKIP_ALSA", "ON")
            .define("SURGE_SKIP_STANDALONE", "ON")
            .define("SURGE_SKIP_LUA", "ON")
            .define("CMAKE_EXPORT_COMPILE_COMMANDS", "ON")
            .define("ENABLE_LTO", "OFF")
            //.profile("Debug")
            //.cxxflag("-fno-function-sections")
            //.cxxflag("-fno-data-sections")
            //.cxxflag("-Wl,--no-gc-sections")
            //.cxxflag("--verbose")
            .build();
        (sdst, String::from(bdst.to_str().unwrap()))
    }.to_owned();

    println!("cargo:-rerun-if-changed=wrapper.h");
    println!("cargo:-rerun-if-changed=cpp");

    // i know.
    println!("cargo:rustc-link-search=native={}", bpath.clone() + "/build/src/common");
    println!("cargo:rustc-link-search=native={}", bpath.clone() + "/build/src/lua");
    println!("cargo:rustc-link-search=native={}", bpath.clone() + "/build/libs/zstd/build/cmake/lib");
    println!("cargo:rustc-link-search=native={}", bpath.clone() + "/build/libs/sqlite-3.23.3");
    println!("cargo:rustc-link-search=native={}", bpath.clone() + "/build/libs/oddsound-mts");
    println!("cargo:rustc-link-search=native={}", bpath.clone() + "/build/libs/fmt");
    println!("cargo:rustc-link-search=native={}", bpath.clone() + "/build/libs/pffft");
    println!("cargo:rustc-link-search=native={}", bpath.clone() + "/build/libs/eurorack");
    println!("cargo:rustc-link-search=native={}", bpath.clone() + "/build/libs/binn");
    println!("cargo:rustc-link-search=native={}", bpath.clone() + "/build/libs/airwindows");
    println!("cargo:rustc-link-search=native={}", bpath.clone() + "/build/libs/sst/sst-plugininfra");
    println!("cargo:rustc-link-search=native={}", bpath.clone() + "/build/libs/sst/sst-plugininfra/libs/strnatcmp");
    println!("cargo:rustc-link-search=native={}", bpath.clone() + "/build/libs/sst/sst-plugininfra/libs/tinyxml");
    println!("cargo:rustc-link-lib=static=surge-lua-src");
    println!("cargo:rustc-link-lib=static=surge-common");
    println!("cargo:rustc-link-lib=static=zstd");
    println!("cargo:rustc-link-lib=static=sqlite");
    println!("cargo:rustc-link-lib=static=oddsound-mts");
    println!("cargo:rustc-link-lib=static={}", if env::var("OPT_LEVEL").unwrap() != "0" { "fmt" } else { "fmtd" });  // why.
    println!("cargo:rustc-link-lib=static=pffft");
    println!("cargo:rustc-link-lib=static=eurorack");
    println!("cargo:rustc-link-lib=static=binn");
    println!("cargo:rustc-link-lib=static=airwindows");
    println!("cargo:rustc-link-lib=static=sst-plugininfra");
    println!("cargo:rustc-link-lib=static=strnatcmp");
    println!("cargo:rustc-link-lib=static=tinyxml");

    println!("cargo:rerun-if-changed=cpp/bridge.cpp");
    println!("cargo:rerun-if-changed=cpp/bridge.h");
    let mut bbuild = cc::Build::new();
    bbuild
        .warnings(false)
        .cpp(true)
        .std("c++20")
        //.opt_level(0)
        .include(spath.clone())
	.flag("-fno-char8_t")          // read ahead. this has to go here too...
        //.flag("-fno-function-sections")
        //.flag("-fno-data-sections")
        //.flag("-Wl,--no-gc-sections")
        //.flag("-fvisibility=default")
        //.flag("--verbose")
        .file("cpp/bridge.cpp");

    let comcom = bpath.clone() + "/build/compile_commands.json";    // "compile commands". comcom.
    let json = std::fs::read_to_string(&comcom).expect("failed to read comcom!");
    let coms: serde_json::Value = serde_json::from_str(&json).expect("failed to parse comcom!");

    let mut bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-I".to_owned() + &spath)    // crazy you gotta do this owned stuff.
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
        .allowlist_item(".*Storage.*")      // fix for surge storage (most stuff).
        .allowlist_item(".*State.*")        // fix for surge storage (other stuff).
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new().rerun_on_header_files(false)));

    // get and use all the include paths from the configure.
    let mut unique = HashSet::new();
    for entry in coms.as_array().unwrap() {
        if let Some(clist) = entry.get("command") {
            shell_words::split(clist.as_str().unwrap())
                .unwrap()
                .into_iter()
                .filter(|x| x.starts_with("-I") || x.starts_with("-D"))
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
