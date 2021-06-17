use directories::UserDirs;
use error_chain::error_chain;
use os_info::get;
use std::env;
use unzpack::Unzpack;

mod lib;

error_chain! {
     foreign_links {
         Io(std::io::Error);
         HttpRequest(reqwest::Error);
     }
}

struct Aerondight {
    download_url: String,
}

impl std::default::Default for Aerondight {
    fn default() -> Self {
        Self {
            download_url: format!(
                "https://dl.google.com/android/repository/commandlinetools-{}-{}_latest.zip",
                Self::get_os_type(),
                Self::get_version(),
            ),
        }
    }
}

impl Aerondight {
    fn get_os_type() -> String {
        let type_ = get().os_type();
        if type_.to_string() == "Mac OS".to_string() {
            String::from("mac")
        } else if type_.to_string() == "Windows".to_string() {
            String::from("win")
        } else {
            String::from("linux")
        }
    }

    fn get_version() -> String {
        String::from("7302050")
    }
}

async fn download_commandlinetools() -> Result<()> {
    if let Some(user_dirs) = UserDirs::new() {
        let aerondight_config = Aerondight::default();

        // fetch handler
        lib::fetch(
            aerondight_config.download_url,
            String::from(format!(
                "{}/{}",
                user_dirs.download_dir().unwrap().display(),
                "android-sdk.zip"
            )),
        )
        .await
        .unwrap();

        // unzip handler
        let download_dir_to_display = user_dirs.download_dir().unwrap().display();
        Unzpack::extract(
            format!("{}/{}", download_dir_to_display, "android-sdk.zip"),
            format!("{}/{}", download_dir_to_display, "android-sdk"),
        )
        .unwrap();
    };
    Ok(())
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(x) => {
            if x == "install" {
                if let Some(user_dirs) = UserDirs::new() {
                    let cmd_to_list_download: &str =
                        &format!("ls {}", user_dirs.download_dir().unwrap().display()).to_owned();
                    download_commandlinetools().await.unwrap();
                    lib::exec(cmd_to_list_download);
                    lib::exec("avdmanager list target")
                }
            }
        }
        _ => println!("Usage: aerondight install"),
    }
}
