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
    download_url: String
}

impl std::default::Default for Aerondight {
    fn default() -> Self {
        Self {
            download_url: format!(
                "https://dl.google.com/android/repository/commandlinetools-{}-{}_latest.zip",
                Self::get_os_type(),
                Self::get_version(),
            )
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

    fn setup_android_path() -> String {
        if let Some(user_dirs) = UserDirs::new() {
            env::set_var("android_path", format!("{}/Library/Android/sdk", user_dirs.home_dir().display()));
            env::var("android_path").unwrap().to_string()
        } else {
            String::new()
        }
    }
}

async fn download_and_unzip_cmdlinetools() -> Result<()> {
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
    Aerondight::setup_android_path();
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(x) => {
            if x == "install" {
                // check java binary in your machine, if exists skip installation progcess
                match lib::find_it("java") {
                    Some(x) => println!("Installed: {:?} ✅", x),
                    None => {
                        // install adoptopenjdk8
                        lib::exec("brew search adoptopenjdk/openjdk/adoptopenjdk8");
                        lib::exec("brew install adoptopenjdk/openjdk/adoptopenjdk8")
                    }
                };

                // check sdkmanager binary in your machine, if exists skip installation process
                match lib::find_it("sdkmanager") {
                    Some(x) => println!("Installed: {:?} ✅", x),
                    None => {
                        // check local user directories base on your OS.
                        if let Some(user_dirs) = UserDirs::new() {
                            download_and_unzip_cmdlinetools().await.unwrap();

                            let display_download_dir = user_dirs.download_dir().unwrap().display();
                            let cmd_to_list_download: &str =
                                &format!("ls {}", display_download_dir).to_owned();
                            lib::exec(cmd_to_list_download);

                            use std::fs::{copy, create_dir_all};

                            let lib_directory =
                                format!("{}/Library", user_dirs.home_dir().display());

                            // create directory '$HOME/Android/sdk.cmdline-tools/latest'
                            create_dir_all(format!(
                                "{}/Android/sdk/cmdline-tools/latest",
                                lib_directory
                            ))
                            .unwrap();
                            // copy android-sdk from dowload directory to lib directory.
                            copy(
                                format!("{}/android-sdk/cmdline-tools", display_download_dir),
                                format!("{}/Android/sdk/cmdline-tools/latest", lib_directory),
                            )
                            .unwrap();
                            // show list of cmdline-tools
                            let cmd_to_list_sdk: &str =
                                &format!("ls {}/Android/sdk/cmdline-tools/latest", lib_directory)
                                    .to_owned();
                            lib::exec(cmd_to_list_sdk)
                        }
                    }
                }
            }
        }
        _ => println!("Usage: aerondight install"),
    }
}
