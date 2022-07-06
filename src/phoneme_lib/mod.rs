//! a lib to map word to phoneme
use super::tools::*;
use std::collections::BTreeSet;
use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io;
use std::io::prelude::*;
use std::path::Path;

///generate word list from files like processed lrc etc.,
///and sort them by lexicographical order
///
/// if you have .lrc files, process them using process_lrc from lyric_process mod
///
/// if write_to_file is true, will write to file_name_word_list.txt, which is defined by
/// the original library.
pub fn gen_word_list<P>(file_name: P, write_to_file: bool) -> io::Result<WordList>
where
    P: AsRef<Path> + std::fmt::Display + Clone,
{
    //TODO: Change to simpler iter
    let mut set = BTreeSet::new();
    let file_lines = read_lines(&file_name)?;
    file_lines
        .into_iter()
        .map(|item| item.unwrap())
        .map(|line| {
            for word in line.split_whitespace() {
                set.insert(word.to_owned()); //is to_owned a must?
            }
        })
        .for_each(drop);
    if write_to_file {
        let final_path = Path::new(&format!("{}_word_list.txt", file_name)).to_owned();
        let mut output_file = File::create(&final_path)?;
        set.iter()
            .map(|word| output_file.write(word.as_bytes()))
            .for_each(drop);
        Ok(WordList::Outer(Box::new(final_path)))
    } else {
        Ok(WordList::Inner(Box::new(set)))
    }
}

///generate dictionary from cmu dict or BigCiDian, the result will be stored to map dict.
pub fn make_dic(path: &Path, dic: &mut HashMap<String, String>) -> io::Result<()> {
    println!("using dict {}", path.display());
    let lines = read_lines(path)?;
    for line in lines {
        let line = line?;
        let all = line.split('\t').collect::<Vec<&str>>();
        match all.len() {
            2 => {
                dic.insert(all[0].trim().to_string(), all[1].to_string());
                // println!("add {} {}", all[0].trim().to_string(), all[1].to_string())
            }
            _ => {
                println!("{:?} the line contains less than two words, skip", all);
            }
        }
    }
    Ok(())
}

fn match_and_write(
    file: &Path,
    dic: &HashMap<String, String>,
    missing: &mut HashSet<String>,
    out: &str,
) -> io::Result<()> {
    let lines = read_lines(file).unwrap_or_else(|_| panic!("can't open {:?}!", file.display()));
    //dataset0042_word2phonemes
    println!("write to {}", &out);
    let mut outfile = File::create(out).expect("invalid outfile!");
    for word in lines {
        let mut word = word?.trim().to_string();
        //write to format as "word\tphonemes"
        if word == *"\n" {
            continue;
        }
        match dic.get(&word) {
            Some(phonemes) => {
                word.push('\t');
                let mut phonemes_str = phonemes.to_string();
                phonemes_str.push('\n');
                outfile
                    .write_all(word.as_bytes())
                    .expect("invalid outfile!");
                outfile
                    .write_all(phonemes_str.as_bytes())
                    .expect("can't write!");
            }
            None => {
                println!("no word {} ", word);
                missing.insert(word);
            }
        };
    }
    Ok(())
}

///look up word from `path_of_word` using dict `dic`
///
/// use path_of_word as the word list directory, or use a BTreeSet to store words
///
/// assume the word file is named by: datasetname_subfix,  
/// and subfix have a default value of "_word_list"
///
/// the path of word should be a directory path
pub fn write_to_words<P>(
    dic: &HashMap<String, String>,
    path_of_word: &WordList,
    path_of_output: P,
    input_subfix: Option<&str>,
) -> io::Result<()>
where
    P: AsRef<Path>,
{
    let mut missing = HashSet::new();
    match path_of_word {
        WordList::Inner(word_list) => {
            let mut outfile =
                File::create(path_of_output.as_ref().join("dataset1_word2phonemes.txt"))?;
            for word in word_list.iter() {
                match dic.get(word) {
                    Some(phonemes) => {
                        let tmp = word.to_string() + "\t" + phonemes + "\n";
                        outfile.write_all(tmp.as_bytes()).expect("invalid outfile!");
                    }
                    None => {
                        println!("no word {} ", word);
                        missing.insert(word.clone());
                    }
                };
            }
        }
        WordList::Outer(path_of_word) => {
            let data_subfix = input_subfix.unwrap_or("_word_list");
            let file_meta = fs::metadata(path_of_word.as_path())?;
            if file_meta.is_dir() {
                let all_file = fs::read_dir(path_of_word.as_ref())?;
                for file in all_file {
                    //dataset0002_word_list
                    let file = file?;
                    if file.metadata().unwrap().is_dir() {
                        continue;
                    }
                    let file_name = file
                        .file_name()
                        .into_string()
                        .expect("not utf-8, invalid file name!");
                    let dataset_name = match file_name.find(data_subfix) {
                        Some(num) => file_name[..num].to_string(),
                        None => {
                            println!(
                                r#"file {} do not match the "{}" pattern "#,
                                file_name, data_subfix
                            );
                            continue;
                        }
                    };
                    let out = format!(
                        "{}/{}_word2phonemes.txt",
                        path_of_word.display(),
                        dataset_name
                    );
                    match_and_write(&file.path(), dic, &mut missing, &out)?;
                }
            } else {
                let file_name = path_of_word.as_path();
                let file_string = file_name.to_string_lossy();
                let file_prefix = file_string.split('.').collect::<Vec<_>>();
                let output_file_name = file_prefix[0].to_string() + "_word2phonemes.txt";
                match_and_write(file_name, dic, &mut missing, &output_file_name)?;
            }
        }
    };
    let mut missing_file = File::create(path_of_output.as_ref().join("missing.txt"))?;
    for mut word in missing {
        word.push('\n');
        missing_file.write_all(word.as_bytes()).unwrap();
    }
    Ok(())
}

#[test]
fn test_write_to_words() {
    use tempdir::TempDir;
    let mut dic_mine = HashMap::new();
    let mut word_list = BTreeSet::new();
    let dir = TempDir::new("test").unwrap();

    dic_mine.insert("hello".to_string(), "HH AH L OW".to_string());
    word_list.insert("hello".to_string());
    word_list.insert("foobar".to_string());
    if let Err(err) = write_to_words(
        &dic_mine,
        &WordList::Inner(Box::new(word_list)),
        dir.path(),
        None,
    ) {
        println!("this {:?}", err)
    }
    let mut output_file = File::open(dir.path().join("dataset1_word2phonemes.txt")).unwrap();
    let mut output_thing = "".to_string();
    output_file.read_to_string(&mut output_thing).unwrap();
    assert_eq!(output_thing, "hello\tHH AH L OW\n");
    let mut missing_file = File::open(dir.path().join("missing.txt")).unwrap();
    let mut missing_thing = "".to_string();
    missing_file.read_to_string(&mut missing_thing).unwrap();
    assert_eq!(missing_thing, "foobar\n".to_string());

    dir.close().unwrap();
}
