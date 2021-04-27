use std::env;
use gdsm::{GameSave};
use std::process::exit;
use clap::{Clap, AppSettings};

enum Function{
    Help,
    List
}

#[derive(Clap)]
#[clap(version = "0.1")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts
{
    save_dir: String
}

fn main() {

    let opts: Opts = Opts::parse();

    let game = GameSave::new(&opts.save_dir);

    let levels = game.get_local_levels();
    levels.into_iter().for_each(|l| println!("{}", l.name));

}