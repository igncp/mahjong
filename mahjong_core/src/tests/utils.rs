use crate::{Game, Players};
use std::{fs, path::PathBuf};

pub fn fixtures_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../core_spec/test/fixtures")
}

pub fn read_fixture(name: &str) -> (String, String) {
    let fixture_path = fixtures_path().join(name);
    let input = fs::read_to_string(fixture_path.join("input.txt")).unwrap();
    let output = fs::read_to_string(fixture_path.join("output.txt")).unwrap();

    (filter_comments(&input), filter_comments(&output))
}

pub fn fixture_names(prefix: &str) -> Vec<String> {
    let mut names = fs::read_dir(fixtures_path())
        .unwrap()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().unwrap().is_dir())
        .filter_map(|entry| entry.file_name().into_string().ok())
        .filter(|name| name.starts_with(prefix))
        .collect::<Vec<_>>();
    names.sort();
    names
}

fn filter_comments(fixture: &str) -> String {
    fixture
        .lines()
        .filter(|line| !line.starts_with("-- "))
        .collect::<Vec<_>>()
        .join("\n")
}

impl Game {
    pub fn start_with_players(&mut self) {
        self.players = Players::default();
        self.players.push("0".to_string());
        self.players.push("1".to_string());
        self.players.push("2".to_string());
        self.players.push("3".to_string());
        self.start(false);
    }
}
