use std::error::Error;
use std::fs;
use std::fs::create_dir_all;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use chrono::{DateTime, Local};
use dirs::home_dir;

const HIGH_SCORES_MAX_SIZE: usize = 10;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct HighScores {
    path: String,
    entries: Vec<HighScore>
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct HighScore {
    score: u32,
    time: DateTime<Local>,
}

impl HighScore {
    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn time(&self) -> DateTime<Local> {
        self.time
    }
}

impl HighScores {
    pub fn read(path: &str) -> Result<HighScores, Box<dyn Error>> {
        let file = HighScores::file(path.to_string())?;

        if !file.exists() {
            return Ok(HighScores { path: path.to_string(), entries: Vec::new() });
        }

        let mut file = File::open(file)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let scores_r = serde_json::from_str(&contents);

        match scores_r {
            Ok(scores) => Ok(scores),
            Err(e) => Err(e.into())
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let file = HighScores::file(self.path.clone())?;

        create_dir_all(&file.parent().unwrap())?;

        let serialized_r = serde_json::to_string_pretty(self);

        match serialized_r {
            Ok(serialized) => {
                fs::write(file, serialized).map_err(From::from)
            }
            Err(e) => Err(Box::new(e))
        }
    }

    pub fn add(&mut self, score: u32) {
        let entry = HighScore { score, time: Local::now() };
        self.entries.push(entry);
        let len = self.entries.len();
        self.entries.sort_by(|a, b| b.score().cmp(&a.score()));

        if len > HIGH_SCORES_MAX_SIZE {
            self.entries.remove(len - 1);
        }
    }

    pub fn entries(&self) -> Vec<HighScore> {
        self.entries.to_vec()
    }

    pub fn min(&self) -> u32 {
        if self.entries.len() < HIGH_SCORES_MAX_SIZE {
            return 0;
        }

        self.entries.last().map(|it| it.score()).unwrap_or(0)
    }

    pub fn max(&self) -> u32 {
        self.entries.first().map(|it| it.score()).unwrap_or(0)
    }

    fn file(path: String) -> Result<PathBuf, Box<dyn Error>> {
        let mut file = if let Some(home) = home_dir() {
            home
        } else {
            return Err("Impossible to get your home dir!".into());
        };

        file.push(path);
        file.push("scores.json");

        Ok(file)
    }
}

#[cfg(test)]
#[test]
fn given_empty_then_max_should_be_the_minimum_int_value() {
    let hs = HighScores { path: ".test".to_string(), entries: Vec::new() };

    assert_eq!(0, hs.max());
}

#[test]
fn given_empty_then_min_should_be_the_minimum_value() {
    let hs = HighScores { path: ".test".to_string(), entries: Vec::new() };

    assert_eq!(0, hs.min());
}

#[test]
fn given_not_empty_then_max_should_be_the_max_entry_even_when_inserted_in_the_inverse_order() {
    let mut hs = HighScores { path: ".test".to_string(), entries: Vec::new() };

    hs.add(100);
    hs.add(200);

    assert_eq!(200, hs.max());
}

#[test]
fn given_less_entries_then_max_size_then_min_should_be_the_minimum_int_value() {
    let mut hs = HighScores { path: ".test".to_string(), entries: Vec::new() };

    hs.add(100);

    assert_eq!(0, hs.min());
}

#[test]
fn given_enough_entries_then_min_should_be_the_min_entry() {
    let mut hs = HighScores { path: ".test".to_string(), entries: Vec::new() };

    for i in 0..HIGH_SCORES_MAX_SIZE {
        hs.add(i as u32 * 100);
    }

    assert_eq!(0, hs.min());
}

#[test]
fn entries_should_be_sorted_on_add() {
    let mut hs = HighScores { path: ".test".to_string(), entries: Vec::new() };

    hs.add(100);
    hs.add(200);
    hs.add(300);

    let entries = hs.entries();
    assert_eq!(300, entries[0].score());
    assert_eq!(200, entries[1].score());
    assert_eq!(100, entries[2].score());
}

#[test]
fn given_more_entries_than_max_size_then_only_the_higher_entries_should_be_retained() {
    let mut hs = HighScores { path: ".test".to_string(), entries: Vec::new() };

    for i in 0..HIGH_SCORES_MAX_SIZE + 5 {
        hs.add(i as u32 * 100);
    }

    let entries = hs.entries();

    assert_eq!(HIGH_SCORES_MAX_SIZE, entries.len());

    assert_eq!(entries.first().map(HighScore::score), Some(100 * (HIGH_SCORES_MAX_SIZE as u32 + 5 - 1)));
}