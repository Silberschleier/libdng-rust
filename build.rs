extern crate cmake;
use cmake::Config;
use std::env;


fn main() {
    let target  = env::var("TARGET").unwrap();

    let dst = Config::new("src/cpp").build();
    let path = dst.display();

    println!("cargo:rustc-link-search=native={}/lib", path);
    println!("cargo:rustc-link-lib=static=dngbindings");
    println!("cargo:rustc-link-lib=static=dng-sdk");
    println!("cargo:rustc-link-lib=static=xmp-sdk");
    println!("cargo:rustc-link-lib=static=expat");
    println!("cargo:rustc-link-lib=static=jpeg");
    println!("cargo:rustc-link-lib=static=z");


    if target.contains("apple")
    {
        println!("cargo:rustc-link-lib=dylib=c++");
        println!("cargo:rustc-link-search=native={}/build/vcpkg_installed/x64-osx/lib", path);
    }
    else if target.contains("linux")
    {
        println!("cargo:rustc-link-lib=dylib=stdc++");
        println!("cargo:rustc-link-search=native={}/build/vcpkg_installed/x64-linux/lib", path);
    }
    else
    {
        println!("cargo:rustc-link-search=native={}/build/vcpkg_installed/x86-windows/lib", path);
    }
}
