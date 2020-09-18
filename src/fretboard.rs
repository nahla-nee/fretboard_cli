use crate::note::Note;

pub struct Fretboard {
    strings: Vec<Note>,
    fret_count: i32
}

impl Fretboard {
    pub fn new(strings: Vec<Note>, fret_count: i32) -> Fretboard {
        Fretboard{
            strings,
            fret_count
        }
    }

    pub fn get_random_string(&self) -> &Note {
        self.strings.get(rand::random::<usize>()%self.strings.len())
            .expect("Invalid string index accessed")
    }

    pub fn get_random_fret(&self) -> i32 {
        //random number is unsigned because negative frets aren't a thing
        //but our note implementation does adds with i32
        (rand::random::<u32>() % self.fret_count as u32) as i32
    }
}
