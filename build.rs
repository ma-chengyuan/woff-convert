use std::{path::Path, process::Command};

fn main() {
    println!("cargo:rerun-if-changed=CMakeLists.txt");
    println!("cargo:rerun-if-changed=wrapper.cpp");

    let target = std::env::var("TARGET").unwrap();
    let dst = if target != "wasm32-unknown-unknown" {
        // Life is easy
        cmake::Config::new(".").build()
    } else {
        fn emtool(name: &str) -> Command {
            let path = std::env::var(name.to_uppercase()).unwrap_or_else(|_| {
                if cfg!(windows) {
                    format!("{}.bat", name)
                } else {
                    name.to_string()
                }
            });
            Command::new(path)
        }

        let out_dir = std::env::var("OUT_DIR").unwrap();
        let dst = Path::new(&out_dir).join("build");

        let status = emtool("emcmake")
            .arg("cmake")
            .arg(std::env::var("CARGO_MANIFEST_DIR").unwrap())
            .arg("-B")
            .arg(&dst)
            .arg("-DCMAKE_BUILD_TYPE=Release")
            .arg(format!("-DCMAKE_INSTALL_PREFIX={}", dst.display()))
            .args(["-G", "Unix Makefiles"])
            .status();
        match status {
            Ok(status) if !status.success() => panic!("Failed to run cmake"),
            Err(e) => panic!("Failed to run cmake: {}", e),
            Ok(_) => {}
        }
        let status = emtool("emmake")
            .current_dir(&dst)
            .args(["make", "install"])
            .arg("-j")
            .arg(num_cpus::get().to_string())
            .status();
        match status {
            Ok(status) if !status.success() => panic!("Failed to run make"),
            Err(e) => panic!("Failed to run make: {}", e),
            Ok(_) => {}
        }
        dst
    };
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );

    println!("cargo:rustc-link-lib=static=woff2wrapper");
    println!("cargo:rustc-link-lib=static=woff2enc");
    println!("cargo:rustc-link-lib=static=woff2dec");
    println!("cargo:rustc-link-lib=static=woff2common");
    println!("cargo:rustc-link-lib=static=brotlienc-static");
    println!("cargo:rustc-link-lib=static=brotlidec-static");
    println!("cargo:rustc-link-lib=static=brotlicommon-static");
}
