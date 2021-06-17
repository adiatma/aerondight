use error_chain::error_chain;
use indicatif::ProgressBar;
use std::process::Command;

error_chain! {
     foreign_links {
         Io(std::io::Error);
         HttpRequest(reqwest::Error);
     }
}

fn _cmd(base: &'static str, args: Option<Vec<&str>>) {
    let output = match args {
        Some(arg) => Command::new(base)
            .args(arg)
            .output()
            .unwrap(),
        _ => Command::new(base)
            .output()
            .unwrap()
    };

    if output.status.success() {
        let res_success = String::from_utf8_lossy(&output.stdout);
        print!("{}", res_success)
    } else {
        let res_error = String::from_utf8_lossy(&output.stderr);
        print!("{}", res_error)
    }
}

// Example usage:
// exec("adb devices")
pub fn exec(command: &'static str) {
    let cmd: Vec<&str> = command.split(" ").collect();

    if cmd.len() > 1 {
        let filtered_arguments = command
            .split(" ")
            .enumerate()
            .filter(|&(index, _)| index != 0)
            .map(|(_, c)| c);

        let args: Vec<&str> = filtered_arguments.collect();
        _cmd(cmd.get(0).unwrap(), Some(args))
    } else {
        _cmd(cmd.get(0).unwrap(), None)
    }
}

pub async fn fetch(url: String, file_name: String) -> Result<()> {
    // Plain progress bar, totaling 1024 steps.
    let steps = 1024;
    let pb = ProgressBar::new(steps);

    // incrementing one step of the progress bar each time.
    let mut intv = tokio::time::interval(std::time::Duration::from_millis(15));
    for _ in 0..steps {
        intv.tick().await;
        pb.inc(1);
    }

    // handle download file with the url
    let res = reqwest::get(url).await?;
    let mut file = std::fs::File::create(file_name)?;
    let mut content = std::io::Cursor::new(res.bytes().await?);
    std::io::copy(&mut content, &mut file)?;

    // Mark the progress bar as finished.
    pb.finish();
    Ok(())
}
