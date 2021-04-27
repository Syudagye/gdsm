use std::env;
use gdsm::{GameSave};
use std::process::exit;
use clap::{Clap, AppSettings};
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};
use termion::*;
use termion::input::{TermRead, Events};
use termion::event::{Key, Event};

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

    //let levels = game.get_local_levels();
    //levels.into_iter().for_each(|l| println!("{}", l.name));

    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdin = async_stdin();

    write!(stdout, "{}{}", clear::All, cursor::Hide);

    stdout.flush().unwrap();

    let mut exit = false;
    let e = &mut stdin.events();
    loop {

        write!(stdout, "{}{}", termion::cursor::Goto(1, 1), termion::clear::CurrentLine).unwrap();

        e.for_each(|e| {
            match e.unwrap() {
                Event::Key(me) => {
                    match me {
                        Key::Char('q') => exit = true,
                        _ => ()
                    }
                }
                _ => ()
            }
        });

        if exit {
            break;
        }
    }

    write!(stdout, "{}{}", clear::All, cursor::Show).unwrap();
    println!("{:?}", terminal_size().unwrap());

}