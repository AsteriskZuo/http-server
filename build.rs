extern crate bindgen;
extern crate cc;
extern crate protoc_rust;

use std::env;
use std::path::PathBuf;

fn main() {
    build_proto_file();
    build_route_c();
}

fn build_route_c() {
    cc::Build::new()
        .file("src/addon/api_server/route_c/library-bridge.cpp")
        .cpp(true)
        .cpp_link_stdlib("stdc++")
        .cpp_set_stdlib("c++")
        .flag("-std=c++11")
        .compile("route");
    println!("cargo:rerun-if-changed=src/addon/api_server/route_c/library-bridge.hpp");
    println!("cargo:rerun-if-changed=src/addon/api_server/route_c/library-bridge.cpp");
    // println!("cargo:rerun-if-changed=src/c/lib/routing_server.h");
    println!("cargo:rustc-link-search=src/addon/api_server/route_c");
    println!("cargo:rustc-link-lib=dylib=rout_server");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindgen::Builder::default()
        .header("src/addon/api_server/route_c/library-bridge.hpp")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("route.rs"))
        .expect("Couldn't write bindings!");
    let to = out_path
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("librout_server.dylib");
    let output_dir = out_path
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let source_library_file = "src/addon/api_server/route_c/librout_server.dylib";
    let dest_library_file =
        String::from(output_dir.as_os_str().to_str().expect("dest_library_file"))
            + "/librout_server.dylib";
    let source_config_file = "config.yaml";
    let dest_config_file =
        String::from(output_dir.as_os_str().to_str().expect("dest_config_file")) + "/config.yaml";
    println!(
        "{:?}:{:?}:{:?}:{:?}:{:?}",
        output_dir, source_library_file, dest_library_file, source_config_file, dest_config_file
    );
    let ret = std::fs::copy(source_library_file, dest_library_file);
    match ret {
        Ok(_) => (),
        Err(_) => panic!("error"),
    }
    let ret = std::fs::copy(source_config_file, dest_config_file);
    match ret {
        Ok(_) => (),
        Err(_) => panic!("error"),
    }
}

fn build_proto_file() {
    protoc_rust::Codegen::new()
        .out_dir("src/protos")
        .inputs(&[
            "protos/route_client_param.proto",
            "protos/route_common.proto",
            "protos/route_result.proto",
            "protos/route_server_param.proto",
        ])
        .include("protos")
        .run()
        .expect("protoc");
}
