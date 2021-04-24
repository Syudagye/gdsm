mod utils;
mod model;

use std::path::Path;
use std::fs;
use std::panic::panic_any;
use base64::decode_config;
use libflate::gzip::Decoder;
use crate::utils::string_xor_cipher;
use crate::model::level::Level;
use regex::Regex;
use std::io::Read;

pub struct GameSave
{
    dir: String
}

impl GameSave
{
    pub fn new(game_dir: &str) -> GameSave
    {
        let path = Path::new(game_dir);

        if !path.exists() || !path.is_dir(){
            panic!("the directory {} doesn't exists or isn't a directory", game_dir)
        }

        GameSave {
            dir: path.to_str().unwrap().to_owned()
        }
    }

    pub fn get_local_levels(&self) -> Vec<Level>
    {
        let raw = decode_save_file(&*format!("{}CCLocalLevels.dat", &self.dir));

        println!("Searching");
        // Regex stolen from gdshare
        let expr = Regex::new("<k>k_\\d+</k>.+?</d>\n? *</d>").unwrap();
        let mat = expr.find_iter(&raw);

        let mut levels: Vec<Level> = Vec::new();
        mat.for_each(|m| {
            let lvl_str = m.as_str();

            let name = Regex::new("<k>k2</k><s>.+</s>").unwrap().find(lvl_str).unwrap().as_str();
            let name = &name[12..name.find("</s>").unwrap()];

            levels.push(Level {
                name: name.to_owned(),
                raw: lvl_str.to_owned()
            });
        });

        levels
    }

    pub fn get_dir(&self) -> &str
    {
        &self.dir
    }
}

fn decode_save_file(path: &str) -> String // Sputnix inspired
{
    println!("Decoding {}", path);
    let data = fs::read_to_string(path).unwrap_or_else(|_| panic!("Could not load {}", path));

    println!("processing xor operation...");
    let mut xor_data = string_xor_cipher(&data, 0xB);
    xor_data.pop(); // I need this because the last char is not valid utf-8 apparently

    println!("Base64 decoding...");
    let b64_data = decode_config(xor_data, base64::URL_SAFE).unwrap();

    println!("gzip decoding...");
    let mut decoder = Decoder::new(b64_data.as_slice()).unwrap();
    let mut buf = Vec::new();
    decoder.read_to_end(&mut buf);

    println!("done !");
    String::from_utf8(buf).unwrap()
}
