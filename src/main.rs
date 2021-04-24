use std::env;
use gdsm::{GameSave};
use std::process::exit;

enum Function{
    Help,
    List
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let mut game_dir: Option<&str> = None;

    let mut function = Function::Help;

    for i in 0..args.len() {
        match args[i].as_str() {
            "-d" => game_dir = Some(args[i + 1].as_str()),
            "-h" | "--help" => function = Function::Help,
            "-l" | "--list" => function = Function::List,
            _ => ()
        }
    }

    if game_dir == None {
        panic!("No game directory specified, use -d for now")
    }

    let game = GameSave::new(game_dir.unwrap_or(""));

    match function {
        Function::List => {
            let levels = game.get_local_levels();
            levels.into_iter().for_each(|l| println!("{}", l.name))
        },
        Function::Help => {
            println!("Future help menu");
        }
    }
}