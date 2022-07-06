use super::tools::*;
use lrc::Lyrics;
use std::collections::BTreeSet;
use std::fmt::Display;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
extern crate lrc;

pub fn process_lrc<P>(file_name: P, write_to_file: Option<bool>) -> io::Result<WordList>
where
    P: AsRef<Path> + Display,
{
    let to_file = write_to_file.unwrap_or(true);
    let raw_lyrc = fs::read_to_string(&file_name)?;
    let lyc = match Lyrics::from_str(&raw_lyrc) {
        Ok(data) => data,
        Err(err) => {
            println!("{:?}", err);
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("{} not found", &file_name),
            ));
        }
    };
    let mut set = Box::new(BTreeSet::new());
    for line in lyc.get_lines() {
        for words in line.split(' ') {
            set.insert(words.to_string());
        }
    }
    if to_file {
        let name = file_name.to_string();
        let final_path_vec: Vec<&str> = name.split_whitespace().collect();
        let final_path = final_path_vec[0].to_owned() + "_word_list.txt";
        let mut word_list = File::create(&final_path)?;
        for mut word in *set {
            word.push('\n');
            word_list.write_all(word.as_bytes())?;
        }
        Ok(WordList::Outer(Box::new(PathBuf::from(final_path))))
    } else {
        Ok(WordList::Inner(set))
    }
}
