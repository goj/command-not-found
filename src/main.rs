use sqlite::Connection;
use std::{env, process};

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

trait PackageQueries {
    fn select_packages(&self, cmd: &str) -> Result<Vec<String>, sqlite::Error>;
}

impl PackageQueries for Connection {
    fn select_packages(&self, cmd: &str) -> Result<Vec<String>, sqlite::Error> {
        self.prepare("SELECT package FROM Programs WHERE name=:name AND system=:system")?
            .bind_by_name(":name", cmd)?
            .bind_by_name(":system", SYSTEM)?
            .into_cursor()
            .map(|row_res| row_res.map(|row| row.get::<String, _>(0)))
            .collect()
    }
}

fn exec_it(argv: &Vec<String>) {
    let cmd = &argv[0];

    let connection = sqlite::open(DB_PATH).expect("Connecting to DB failed");

    let packages: Vec<String> = connection.select_packages(cmd).expect("Query failed");
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

#[cfg(test)]
mod test {
    use super::*;
    use std::error::Error;

    trait TestHelpers {
        fn add_program(
            &self,
            name: &str,
            package: &str,
            system: &str,
        ) -> Result<(), Box<dyn Error>>;
    }

    fn open_test_db() -> Result<Connection, Box<dyn Error>> {
        let connection = sqlite::open(":memory:").expect("Connecting to DB failed");
        connection.execute("CREATE TABLE Programs (name TEXT, package TEXT, system TEXT)")?;
        Ok(connection)
    }

    impl TestHelpers for Connection {
        fn add_program(
            self: &Connection,
            name: &str,
            package: &str,
            system: &str,
        ) -> Result<(), Box<dyn Error>> {
            self.prepare("INSERT INTO Programs VALUES (:name, :package, :system)")?
                .bind_by_name(":name", name)?
                .bind_by_name(":package", package)?
                .bind_by_name(":system", system)?
                .into_cursor()
                .count();
            Ok(())
        }
    }

    fn test_select_packages() -> Result<(), Box<dyn Error>> {
        let connection = open_test_db()?;
        connection.add_program("bash", "bash", "x86_64-linux")?;
        connection.add_program("bash", "bash-test", "x86_64-linux")?;
        connection.add_program("zsh", "zsh", "x86_64-linux")?;
        connection.add_program("bash", "bash-other-system", "i386-fuchsia")?;
        let packages = connection.select_packages("bash")?;
        assert!(packages.iter().any(|p| p == "bash"));
        assert!(packages.iter().any(|p| p == "bash-test"));
        assert!(!packages.iter().any(|p| p == "zsh"));
        assert!(!packages.iter().any(|p| p == "bash-other-system"));
        Ok(())
    }
}
