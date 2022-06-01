use std::collections::{HashMap};
use std::fs::{File, self, FileType};
use std::path::Path;
use std::io::{self, BufRead};
use std::io::prelude::*;
use std::env;
use std::process::exit;


fn read_lines<P>(file_name: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(file_name)?;
    Ok(io::BufReader::new(file).lines())
}

fn make_dic(path: &Path, dic: &mut HashMap<String, String>) -> io::Result<()>{
    println!("using dict {}", path.display());
    let lines = match read_lines(path) {
        Ok(lins) => lins,
        Err(t) => {
            println!("failed due to {}", t);
            return Err(t)
        }
    };
    for line in lines {
        let line = line?;
        let all = line.split('\t').collect::<Vec<&str>>();
        match all.len() {
            2 => {
                dic.insert(all[0].trim().to_string(), all[1].to_string());
                // println!("add {} {}", all[0].trim().to_string(), all[1].to_string())
                }
            _ => {println!("{:?} the line contains less than two words, skip", all);}
        }
    }
    Ok(())
}

fn get_cmu_dict()->Result<(), String>{
    Ok(())
}

fn write_to_words(dic:&HashMap<String, String>, path_of_word: &Path, path_of_lyrc: &Path) -> io::Result<()> {
    let all_file = fs::read_dir(path_of_word)?;
    for file in all_file {
        //dataset0002_word_list
        let file = file?;
        let data = fs::metadata(file.path())?;
        if !data.is_file(){
            continue;
        }
        let file_name = file.file_name().into_string().expect("invalid file name!");
        let dataset_name = 
            match file_name.find("_word_list"){
                Some(num) => {
                    file_name[..num].to_string()
                }
                None => {
                    continue;
                }
            };
        println!("into dataset {}", dataset_name);
        let full_path = file.path().to_str().unwrap().to_string();
        let path_of_words = Path::new(&full_path);

        let lines = read_lines(path_of_words)
                .expect(&format!("can't open {:?}!", &path_of_words));
        //dataset0042_word2phonemes

        let out = format!("{}/{}_word2phonemes.txt", path_of_word.display().to_string(), dataset_name);
        println!("write to {}", &out);
        let mut outfile = File::create(out).expect("invalid outfile!");
        for word in lines {
            let mut word = word?.trim().to_string();
            //write to format as "w phonemes"
            if let Some(phonemes) = dic.get(&word) {
                word.push_str(" ");
                let mut phonemes_str = phonemes.to_string();
                phonemes_str.push_str("\n");
                outfile.write_all(word.as_bytes()).expect("invalid outfile!");
                outfile.write_all(phonemes_str.as_bytes()).expect("can't write!");
            }
            else {
                println!("no word {} ", word);
            }
        }
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1{
        println!("usage: make_dict dictfile lyric_dir wordfile_dir");
        exit(1);
    }
    let dictfile = env::args().nth(1).expect("no dictfile!");
    let lyric_dir = env::args().nth(2).expect("no lyricdir!");
    let wordfile_dir = env::args().nth(3).expect("no wordfile dir!");

    let path_of_dict = Path::new(&dictfile);
    let path_of_word = Path::new(&wordfile_dir);
    let path_of_lyric = Path::new(&lyric_dir);

    let mut dic = HashMap::new();
    match make_dic(path_of_dict, &mut dic) {
        Ok(()) => {
            println!("diction fine!");
        }
        Err(t) => {
            println!("can not write to dic because of {}", t);
        }
    }
    write_to_words(&dic, path_of_word, path_of_lyric).unwrap();

}
