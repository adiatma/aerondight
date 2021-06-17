use error_chain::error_chain;
use std::{env, path};
use unzpack::Unzpack;

mod lib;

error_chain! {
     foreign_links {
         Io(std::io::Error);
         HttpRequest(reqwest::Error);
     }
}

async fn download_commandlinetools() -> Result<()> {
    let target = "https://dl.google.com/android/repository/commandlinetools-mac-6858069_latest.zip";
    lib::fetch(target.to_string(), "/Users/adiatma/Desktop/android-sdk.zip".to_string())
        .await
        .unwrap();
    Unzpack::extract("/Users/adiatma/Desktop/android-sdk.zip", "/Users/adiatma/Desktop/android-sdk").unwrap();
    Ok(())
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(x) => {
            if x == "install" {
                if path::Path::new("/Users/adiatma/Library/Android").exists() {
                    download_commandlinetools().await.unwrap();
                    lib::exec("ls /Users/adiatma/Desktop")
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
