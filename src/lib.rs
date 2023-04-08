use std::fs;
use std::io::Write;
use std::process;

use termion::cursor;
use termion::event::Key;
use termion::input::TermRead;

pub type Result<T> = std::result::Result<T, std::io::Error>;

struct Line {
    size: usize,
    chars: Vec<char>,
}

impl Line {
    fn new(chars: Vec<char>) -> Self {
        Self {
            size: chars.len(),
            chars: chars,
        }
    }
}

struct EditorContent {
    lenght: usize,
    lines: Vec<Line>,
}

impl EditorContent {
    fn new(lines: Vec<Line>) -> Self {
        Self {
            lenght: lines.len(),
            lines: lines,
        }
    }
}

struct EditorCursor {
    x: u16,
    y: u16,
}

impl EditorCursor {
    fn new() -> Self {
        Self { x: 1, y: 1 }
    }
}

enum EditorMode {
    Normal,
    Command,
    Insert,
    Visual,
}

struct EditorUI {}

impl EditorUI {
    fn new() -> Self {
        Self {}
    }
}

struct EditorOutput {}

impl EditorOutput {
    fn new() -> Self {
        Self {}
    }
}

struct Editor {
    cursor: EditorCursor,
    content: EditorContent,
    mode: EditorMode,
    user_inteface: EditorUI,
    output: EditorOutput,
}

impl Editor {
    fn new(filename: String) -> Self {
        Editor {
            cursor: EditorCursor::new(),
            content: Self::get_content(filename),
            mode: EditorMode::Normal,
            user_inteface: EditorUI::new(),
            output: EditorOutput::new(),
        }
    }

    fn get_content(filename: String) -> EditorContent {
        let lines = fs::read_to_string(filename)
            .unwrap()
            .lines()
            .map(|s| Line::new(s.chars().collect::<Vec<char>>()))
            .collect::<Vec<Line>>();

        EditorContent::new(lines)
    }
}

pub fn parse() -> String {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: atr <FILENAME>");
        process::exit(0)
    } else {
        args[1].to_owned()
    }
}

pub fn run() -> Result<()> {
    Ok(())
}
