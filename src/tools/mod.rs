use std::collections::BTreeSet;
use std::fmt::{self, Display};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

///read lines from file
pub fn read_lines<P>(file_name: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(file_name)?;
    Ok(io::BufReader::new(file).lines())
}

///a dataset to store files, to deal with both inner BTreeSet and outer Path
///
/// both the two is stored in Box, making it easier to transfer
pub enum WordList {
    ///Path to file or directory, will detect automatically
    ///
    ///notice that we assume the path will only contain valid utf-8 char
    Outer(Box<PathBuf>),
    ///a list of all the words to process, no duplicate item
    Inner(Box<BTreeSet<String>>),
}

impl Display for WordList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let WordList::Outer(path) = &self {
            write!(f, "{}", path.as_path().display())
        } else {
            write!(f, "inner word list")
        }
    }
}
