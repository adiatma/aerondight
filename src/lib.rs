use std::process::Command;

fn single_cmd(base: &'static str) {
    let output = Command::new(base)
        .output()
        .unwrap_or_else(|e| panic!("{}", e));

    if output.status.success() {
        let res_success = String::from_utf8_lossy(&output.stdout);
        print!("{}", res_success)
    } else {
        let res_error = String::from_utf8_lossy(&output.stderr);
        print!("{}", res_error)
    }
}

fn cmd_with_args(base: &'static str, args: Vec<&str>) {
    let output = Command::new(base)
        .args(args)
        .output()
        .unwrap_or_else(|e| panic!("{}", e));

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
        cmd_with_args(cmd.get(0).unwrap(), args)
    } else {
        single_cmd(cmd.get(0).unwrap())
    }
}
