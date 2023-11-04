use std::fs;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

fn main() {
    let bluestacks_file = fs::File::open(r"C:\ProgramData\BlueStacks_nxt\bluestacks.conf").unwrap();

    let file = BufReader::new(bluestacks_file);

    'outer: for line in file.lines() {
        for word in line.unwrap().split_whitespace() {
            if word.starts_with("bst.instance.Pie64.status.adb_port") {
                let mut port = word.split('=');
                run_adb_command(port.nth(1).unwrap().to_string().replace("\"", ""));
                break 'outer;
            }
        }
    }
}

fn run_adb_command(port: String) {
    let output = Command::new("adb")
        .args(&["connect", &format!("127.0.0.1:{}", port)])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("Unable to connect to adb");

    if String::from_utf8_lossy(&output.stdout).starts_with("cannot connect") {
        println!("Unable to connect to {}", port);
    } else {
        println!("Connected to {0}", port);
    }
}
