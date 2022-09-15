use std::{env, process};

use sqlite::Value;

const DB_PATH: &str = "/nix/var/nix/profiles/per-user/root/channels/nixos/programs.sqlite";
const SYSTEM: &str = "x86_64-linux";

fn main() {
    let argv: Vec<String> = env::args().skip(1).collect();
    if argv.len() < 1 {
        eprintln!("Must specify command to execute");
        process::exit(127);
    }
    exec_it(&argv)
}

fn exec_it(argv: &Vec<String>) {
    let cmd = &argv[0];

    let connection = sqlite::open(DB_PATH).expect("Connecting to DB failed");

    let packages: Vec<String> = connection
        .prepare("SELECT package FROM Programs WHERE name=? AND system=?")
        .expect("Preparing statement failed.")
        .into_cursor()
        .bind(&[
            Value::String(cmd.to_string()),
            Value::String(SYSTEM.to_string()),
        ])
        .expect("Binding values failed.")
        .filter_map(Result::ok)
        .map(|row| row.get(0))
        .collect();

    if packages.is_empty() {
        println!("{}: command not found", cmd);
    } else {
        print!("The program '{}' is not in your PATH. ", cmd);
        let suffix = if packages.len() == 1 {
            ""
        } else {
            " one of the following"
        };
        println!("You can run it by typing{}:", suffix);
        let args = argv[1..].join(" ");
        for pkg in &packages {
            println!("  nix run nixpkgs#{} {}", pkg, &args);
        }
    }
    process::exit(127);
}
