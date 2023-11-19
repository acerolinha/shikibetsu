mod args;
mod entries_handler;
mod entry;

pub use args::Args;
use entries_handler::EntriesHandler;

pub fn run_with_args(args: &Args) {
    let entries_handler = EntriesHandler::new(args);

    for entry in entries_handler.get_entries(&args.path).iter() {
        println!("{}", entry.display(args));
    }
}

#[cfg(test)]
mod tests {
    use assert_cmd::prelude::*;
    use assert_fs::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    #[test]
    fn it_should_throw_when_path_does_not_exist() {
        let mut cmd = Command::cargo_bin("shikibetsu").unwrap();

        cmd.arg("./this/path/does/not/exist");
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("Failed to read directory"));
    }

    #[test]
    fn it_should_not_include_hidden_files() {
        let mut cmd = Command::cargo_bin("shikibetsu").unwrap();

        let temp = assert_fs::TempDir::new().unwrap();
        temp.child("not_hidden").touch().unwrap();
        temp.child(".hidden").touch().unwrap();

        cmd.arg(temp.path());
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("not_hidden"))
            .stdout(predicate::str::contains(".hidden").not());
    }

    #[test]
    fn it_should_include_hidden_files() {
        let mut cmd = Command::cargo_bin("shikibetsu").unwrap();

        let temp = assert_fs::TempDir::new().unwrap();
        temp.child("not_hidden").touch().unwrap();
        temp.child(".hidden").touch().unwrap();

        cmd.arg(temp.path()).arg("-a");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("not_hidden"))
            .stdout(predicate::str::contains(".hidden"));
    }

    #[test]
    fn it_should_display_emoji_icons() {
        let mut cmd = Command::cargo_bin("shikibetsu").unwrap();

        let temp = assert_fs::TempDir::new().unwrap();
        temp.child("dir").create_dir_all().unwrap();

        let file = temp.child("file");
        file.touch().unwrap();

        temp.child("link_to_file")
            .symlink_to_file(file.path())
            .unwrap();

        cmd.arg(temp.path()).arg("-e");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("📄"))
            .stdout(predicate::str::contains("📁"))
            .stdout(predicate::str::contains("🔗"));
    }
}
