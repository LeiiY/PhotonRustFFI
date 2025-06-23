fn main() {
    
    pkg_config::Config::new()
        .atleast_version("22.11")
        .statik(true)
        .probe("libdpdk")
        .expect("Failed to find libdpdk via pkg-config");
    println!("cargo:rustc-link-search=native=/usr/local/lib/x86_64-linux-gnu");
    println!("cargo:rustc-link-lib=static=rte_net_bond");
    println!("cargo:rustc-link-search=native=/usr/local/lib");
    println!("cargo:rustc-link-lib=static=fstack");

    // 基本 C 系统库
    println!("cargo:rustc-link-lib=pthread");
    println!("cargo:rustc-link-lib=dl");
    println!("cargo:rustc-link-lib=m");
    println!("cargo:rustc-link-lib=rt");
    println!("cargo:rustc-link-lib=numa");
    println!("cargo:rustc-link-lib=crypto");
    println!("cargo:rustc-link-lib=c");

    println!("cargo:rustc-link-search=native=../build");
    println!("cargo:rustc-link-search=native=../build/_deps/photon-build/output");
    println!("cargo:rustc-link-lib=static=net-perf");
    println!("cargo:rustc-link-lib=gflags");
    println!("cargo:rustc-link-lib=static=photon");
    println!("cargo:rustc-link-lib=pthread");
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-lib=dl");
    println!("cargo:rustc-link-lib=rt");
    println!("cargo:rustc-link-lib=ssl");
    println!("cargo:rustc-link-lib=crypto");
    println!("cargo:rustc-link-lib=curl");
    println!("cargo:rustc-link-lib=aio");
} 