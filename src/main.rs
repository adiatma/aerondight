mod lib;

use std::{env, path};

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(x) => {
            if x == "install" {
                if path::Path::new("/Users/adiatma/Library/Android").exists() {
                    println!("Install...")
                }
            }

            if x == "check" {
                lib::exec("adb devices");
                lib::exec("android list target")
            }
        }
        _ => println!("Usage: aerondight <install|check>"),
    }
}
