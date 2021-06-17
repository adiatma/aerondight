use error_chain::error_chain;
use std::{env, path};

mod lib;

error_chain! {
     foreign_links {
         Io(std::io::Error);
         HttpRequest(reqwest::Error);
     }
}

async fn download_commandlinetools() -> Result<()> {
    let target = "https://dl.google.com/android/repository/commandlinetools-mac-6858069_latest.zip";
    lib::fetch(target.to_string(), "android-sdk.zip".to_string())
        .await
        .unwrap();
    Ok(())
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(x) => {
            if x == "install" {
                if path::Path::new("/Users/adiatma/Library/Android").exists() {
                    download_commandlinetools().await.unwrap()
                }
            }

            if x == "check" {
                lib::exec("adb devices");
                lib::exec("android list target");
                lib::exec("ls -a")
            }
        }
        _ => println!("Usage: aerondight <install|check>"),
    }
}
