use std::env::args;
use std::path::PathBuf;
use std::str::FromStr;

mod buffer;
use buffer::Buffer;

// mod renderer;
// use renderer::{NCursesRenderer, Renderer};

mod view;
mod editor;

use view::View;

use termion::{event::Key, raw::RawTerminal};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use std::io::{Write, stdout, stdin, Stdout};

use termion::{color, clear, style, cursor};
use crate::editor::{Editor, EditorError};


fn main() {
    let mut debug: bool = false;
    let argv: Vec<String> = args().into_iter().collect();
    let mut path = None;

    if argv.len() == 0 {
        panic!("Why is argv 0??");
    }
    if argv.len() == 1 {

    }

    if argv.len() == 2 {
        if argv[1] == "debug".to_string() {
            debug = true;
        } else {
            path = Some(PathBuf::from_str(argv[1].as_str()).unwrap());
        }
    }

    let mut editor = Editor::new(path);

    let mut stdout = stdout().into_raw_mode().unwrap();
    // let stdin = termion::async_stdin();
    let stdin = stdin();

    write!(stdout, "{}{}{}", termion::clear::All, termion::cursor::Goto(1, 1), termion::cursor::BlinkingBlock).unwrap();

    editor.render(&mut stdout, debug);
    stdout.flush().unwrap();
    let mut it = stdin.keys();

    loop {
        if let Some(res) = it.next() {
            if let Ok(key) = res {
                if let Err(err) = editor.handle_key(key) {
                    match err {
                        EditorError::Quit => {
                            break;
                        },
                        EditorError::ToggleDebug => {
                            debug = !debug;
                        },
                    }
                }
            } else {
                panic!("Not sure what happened but stdin was invalid for some reason");
            }
            editor.render(&mut stdout, debug);
            stdout.flush().unwrap();
        } else {
        }
    }
    // write!(stdout, "{}", termion::clear::All).unwrap();
    // stdout.flush().unwrap();
}
