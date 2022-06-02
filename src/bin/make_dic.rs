extern crate phoneme_lib;
use phoneme_lib::{phoneme_lib::*};

use std::collections::{HashMap};
use std::path::Path;
use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1{
        println!("usage: make_dict <dictfile> <wordfile_dir>");
        exit(1);
    }
    let dictfile = env::args().nth(1).expect("no dictfile!");
    // let lyric_dir = env::args().nth(2).expect("no lyricdir!");
    let wordfile_dir = env::args().nth(2).expect("no wordfile dir!");

    let path_of_dict = Path::new(&dictfile);
    let path_of_word = Path::new(&wordfile_dir);
    // let path_of_lyric = Path::new(&lyric_dir);

    let mut dic = HashMap::new();
    match make_dic(path_of_dict, &mut dic) {
        Ok(()) => {
            println!("diction fine!");
        }
        Err(t) => {
            println!("can not write to dic because of {}", t);
        }
    }
    write_to_words(&dic, path_of_word).unwrap();
}
