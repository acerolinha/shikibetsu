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
}
