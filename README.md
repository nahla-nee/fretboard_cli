# fretboard_cli

This is a simple command line program I've made to help me learn the notes on
the guitar fretboard, though it works for any fretted instrument.

## Usage
fretboard_cli only has two options, one to specify your instrument's tuning
(`-t` or `--tuning`), and one to specify your instrument's fret count (`-f` or
`--frets`).

`<>` denotes optional flags

`()` denotes required arguments to flags
```
fretboard_cli <-f/--frets> (positive integer) <-t/--tuning> (note names)
```
Simply running
```
fretboard_cli
```
is equivalent to
```
fretboard_cli -f 22 -t E A D G B E
```
The tuning argument can also be used to specify the string count. Here's a 7
string guitar!
```
fretboard_cli -t B E A D G B E
```
and a bass!
```
fretboard_cli -t E A D G
```

## Input
After running the program you will be repeatedly asked to enter the name of the
note on a specific string at a specific fret. You can also exit the program by
typing the word `exit` into the terminal at any point. Here's what that might
look like. Lines starting with `//` are comments inserted to clarify what's
going on.
```
What is the note at the 12th fret of the B string?
B
Correct!
What is the note at the 20th fret of the A string?
A
//if you get an answer wrong you get to try again and again!
Incorrect!
F 
Correct!
What is the note at the 11th fret of the D string?
//you can even use sharps and flats!
Db
Correct!
What is the note at the 17th fret of the E string?
//maybe don't do it like this though
E#####
Correct!
What is the note at the 7th fret of the E string?
//can't really blame you for wanting to leave after that last answer
exit
```