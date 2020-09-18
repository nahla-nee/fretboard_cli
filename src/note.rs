use std::fmt;
use std::convert::TryFrom;

#[derive(Copy, Clone, PartialEq)]
pub enum Note{
    C = 0,
    Cs,
    D,
    Ds,
    E,
    F,
    Fs,
    G,
    Gs,
    A,
    As,
    B
}

impl std::convert::TryFrom<i32> for Note{
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value{
            0 => Ok(Note::C),
            1 => Ok(Note::Cs),
            2 => Ok(Note::D),
            3 => Ok(Note::Ds),
            4 => Ok(Note::E),
            5 => Ok(Note::F),
            6 => Ok(Note::Fs),
            7 => Ok(Note::G),
            8 => Ok(Note::Gs),
            9 => Ok(Note::A),
            10 => Ok(Note::As),
            11 => Ok(Note::B),
            _ => Err("Invalid note input, valid inputs are in the range 0..11")
        }
    }
}

impl std::convert::TryFrom<&str> for Note{
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let err_str = "Invalid note input, recognize inputs are: C, D, E, F, G, \
                       A, B with any number of # or b as modifiers";

        let mut value = value.chars();

        //get letter note, if any
        let mut note_value = match value.next() {
            Some(c) => match c {
                'C' | 'c' => Note::C as i32,
                'D' | 'd' => Note::D as i32,
                'E' | 'e' => Note::E as i32,
                'F' | 'f' => Note::F as i32,
                'G' | 'g' => Note::G as i32,
                'A' | 'a' => Note::A as i32,
                'B' | 'b' => Note::B as i32,
                _ => return Err(err_str)
            },
            None => return Err(err_str)
        };

        for c in value {
            match c {
                '#' => note_value += 1,
                'b' => note_value -= 1,
                _ => return Err(err_str)
            }
        };

        if (note_value%12) < 0 {
            note_value = 12 + note_value;
        }

        Ok(Note::try_from(note_value)
            .expect("Failed to turn note value to enum"))
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self{
            Note::C => "C",
            Note::Cs => "C#",
            Note::D => "D",
            Note::Ds => "D#",
            Note::E => "E",
            Note::F => "F",
            Note::Fs => "F#",
            Note::G => "G",
            Note::Gs => "G#",
            Note::A => "A",
            Note::As => "A#",
            Note::B => "B"
        };

        write!(f, "{}", str)
    }
}

impl std::ops::Add<i32> for &Note {
    type Output = Note;

    fn add(self, rhs: i32) -> Self::Output {
        let mut note = (*self as i32 + rhs)%12;
        if note < 0 {
            note = 12 + note;
        }

        Note::try_from(note)
            .expect("impl Add for Note gave invalid value to Note::from_i32")
    }
}

impl std::ops::Add<i32> for Note {
    type Output = Note;

    fn add(self, rhs: i32) -> Self::Output {
        &self + rhs
    }
}

impl std::ops::Sub<i32> for &Note {
    type Output = Note;

    fn sub(self, rhs: i32) -> Self::Output {
        let mut note = (*self as i32 - rhs)%12;
        if note < 0 {
            note = 12+note;
        }

        Note::try_from(note)
            .expect("impl Sub for Note gave invalid value to Note::from_i32")
    }
}

impl std::ops::Sub<i32> for Note {
    type Output = Note;

    fn sub(self, rhs: i32) -> Self::Output {
        &self - rhs
    }
}
