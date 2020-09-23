mod note;
mod fretboard;

use std::{
    io::{self, Write},
    convert::TryFrom,
    env
};

use crossterm::{
    execute,
    style::{Color, SetForegroundColor, ResetColor, Print}
};
use thiserror::Error;

use note::Note;
use fretboard::Fretboard;

enum UserInput {
    Note (Note),
    Exit
}

#[derive(Error, Debug)]
enum ParseArgsError {
    #[error("--frets or -f was passed without a fret count after")]
    NoFretCount,
    #[error("Failed to parse number ({0}) given to --frets or -f")]
    InvalidFretCount(String),
    #[error("--frets or -f was passed a negative fret count of {0}")]
    NegativeFretCount(i32),
    #[error("Failed to parse note ({0}) given to --tuning or -t")]
    InvalidNote(String),
    #[error("--tuning or -t was passed without any notes")]
    NoNotes,
    #[error("Invalid argument: ({0})")]
    InvalidArgument(String)
}

#[derive(Error, Debug)]
enum UserInputError {
    #[error("Invalid input, accepted inputs: exit or a note name")]
    InvalidInput
}

fn main() {
    let fretboard = match parse_args() {
        Ok(f) => f,
        Err(e) => {
            println!("{}", e);
            return
        }
    };
    let mut run = true;

    while run{
        let string = fretboard.get_random_string();
        let fret = fretboard.get_random_fret();
        let note = string+fret;

        println!("What is the note at the {}{} fret of the {} string?",
                 fret, get_ordinal_suffix(fret), string);

        loop {
            match parse_user_input() {
                Ok(uin) => {
                    match uin {
                        UserInput::Note(answer) => {
                            if answer == note {
                                let _ = execute!(
                                    io::stdout(),
                                    SetForegroundColor(Color::Green),
                                    Print("Correct!\n"),
                                    ResetColor
                                );
                                break
                            }
                            else {
                                let _ = execute!(
                                    io::stdout(),
                                    SetForegroundColor(Color::Red),
                                    Print("Incorrect!\n"),
                                    ResetColor
                                );
                            }
                        }
                        UserInput::Exit => {
                            run = false;
                            break
                        }
                    }
                }
                Err(e) => println!("{}", e)
            }
        }
    }
}

fn parse_args() -> Result<Fretboard, ParseArgsError> {
    let mut args = env::args().peekable();
    let _program_name = args.next()
        .expect("program name somehow doesn't exist how did you even do that?");

    let mut strings = vec![Note::E, Note::A, Note::D, Note::G, Note::B,
                           Note::E];
    let mut fret_count = 22;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--frets" | "-f" => {
                if let Some(num_str) = args.next() {
                    if let Ok(val) = num_str.parse() {
                        if val < 0 {
                            return Err(ParseArgsError::NegativeFretCount(val))
                        }
                        fret_count = val
                    }
                    else {
                        return Err(ParseArgsError::InvalidFretCount(num_str))
                    }
                }
                else {
                    return Err(ParseArgsError::NoFretCount)
                }
            },
            "--tuning" | "-t" => {
                strings = Vec::new();

                //loop until we hit the next cli flag
                while let Some(arg) = args.peek() {
                    if arg.starts_with('-') {
                        break
                    }

                    if let Ok(note) = Note::try_from(arg.as_str()) {
                        strings.push(note);
                        args.next();
                    }
                    else {
                        return Err(ParseArgsError::InvalidNote(arg.clone()))
                    }
                }

                if strings.len() == 0 {
                    return Err(ParseArgsError::NoNotes)
                }
            }
            _ => return Err(ParseArgsError::InvalidArgument(arg))
        }
    };

    Ok(Fretboard::new(strings, fret_count))
}

fn parse_user_input() -> Result<UserInput, UserInputError> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Error reading input from stdin");

    let trimmed_input = input.trim();

    if trimmed_input == "exit"{
        Ok(UserInput::Exit)
    }
    else{
        match Note::try_from(trimmed_input){
            Ok(note) => Ok(UserInput::Note(note)),
            Err(_) => Err(UserInputError::InvalidInput)
        }
    }
}

fn get_ordinal_suffix(num: i32) -> &'static str {
    match (num % 10, num %100){
        (1, 11) | (2, 12) | (3, 13) => "th",
        (1, _) => "st",
        (2, _) => "nd",
        (3, _) => "rd",
        _ => "th"
    }
}
