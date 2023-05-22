use std::env::args;
use std::path::PathBuf;
use std::str::FromStr;

mod buffer;
use buffer::Buffer;


mod view;
mod editor;
mod renderer;

use view::View;

use termion::{event::Key, raw::RawTerminal};
use termion::input::TermRead;
use std::io::stdin;
use crate::editor::{Editor, EditorError};
use crate::renderer::{TerminalRenderer, Renderer};


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

    // let stdin = termion::async_stdin();
    let stdin = stdin();

    let mut editor = Editor::new();
    let mut renderer = TerminalRenderer::new();
    editor.open_file(path);

    renderer.render(&editor);
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
                            renderer.debug = !renderer.debug;
                            debug = !debug;
                        },
                    }
                }
            } else {
                panic!("Not sure what happened but stdin was invalid for some reason");
            }
            renderer.render(&editor);
        } else {
        }
    }
}
