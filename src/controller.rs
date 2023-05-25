use std::{
    env::args,
    io::{stdin, stdout, Write, Stdin},
    path::PathBuf,
    str::FromStr,
};

use termion::{input::TermRead, event::Key};

use crate::{
    editor::Editor,
    renderer::{Renderer, TerminalRenderer}, buffer::{TextObject, Direction},
};

pub enum EditorCommand {
    ToggleDebug,
    Quit,
}

pub enum Redraw {
    All,
    Cursor,
    FocusedView,
    StatusLine,
    CurrentLine,
    BeforeCursor,
    AfterCursor,
}

pub enum ControlCommand {
    Redraw(Redraw),
    Editor(EditorCommand),
    Noop,
}

pub enum EditorState {
    Insert,
}


// (path, debug)
type CLIArgs = (Option<PathBuf>, bool);

pub fn run() {
    let (path, debug) = get_cli_args();

    // let stdin = termion::async_stdin();

    let mut editor = Editor::new();
    let mut renderer = TerminalRenderer::new();
    renderer.debug = debug;
    editor.open_file(path);

    renderer.render(&editor);
    stdout().flush().unwrap();
    let mut it = stdin().keys();

    loop {
        // match &editor.state {
        //     EditorState::Insert => {

        //     }
        //     EditorState::Prompt(message) => {

        //     }
        // }
        let res = it.next().expect("Something went wrong");
        let key = res.expect("Not sure what happened but stdin was invalid for some reason");

        match handle_key(&mut editor, key) {
            ControlCommand::Editor(command) => match command {
                EditorCommand::ToggleDebug => {
                    renderer.debug = !renderer.debug;
                }
                EditorCommand::Quit => {
                    return;
                }
            },
            ControlCommand::Redraw(command) => match command {
                Redraw::All => {
                    renderer.render(&editor);
                }
                Redraw::Cursor => {
                    if debug {
                        renderer.render(&editor);
                    } else {
                    renderer.render_status_line(&editor);
                    renderer.render_cursor(&editor);
                    }
                }
                Redraw::FocusedView => {
                    renderer.render(&editor);
                }
                Redraw::StatusLine => {
                    renderer.render_status_line(&editor);
                }
                Redraw::CurrentLine => {
                }
                Redraw::BeforeCursor => {

                }
                Redraw::AfterCursor => {

                }
            },
            ControlCommand::Noop => {}
        }
        stdout().flush().unwrap();
    }
}

fn get_cli_args() -> CLIArgs {
    let argv: Vec<String> = args().into_iter().collect();

    let mut debug: bool = false;
    let mut path = None;

    if argv.len() == 0 {
        panic!("Why is argv 0??");
    }
    if argv.len() == 1 {}

    if argv.len() == 2 {
        if argv[1] == "debug".to_string() {
            debug = true;
            path = Some(PathBuf::from_str(argv[2].as_str()).unwrap());
        } else {
            path = Some(PathBuf::from_str(argv[1].as_str()).unwrap());
        }
    }

    if argv.len() == 3 {
        if argv[1] == "debug".to_string() {
            debug = true;
        } else {
            panic!("Something went wrong");
        }
        path = Some(PathBuf::from_str(argv[2].as_str()).unwrap());
    }

    (path, debug)
}

pub fn prompt(message: String) -> String {
    let mut it = stdin().keys();

    loop {
        let res = it.next().expect("Something went wrong");
        let key = res.expect("Not sure what happened but stdin was invalid for some reason");
        let mut response = String::new();
        match key {
            Key::Char(c) => {
                match c {
                    '\n' => {

                    }
                    _ => {
                        response.push(c)
                    }
                }
            }
            _ => {}
        }
    }
}


// TODO: Not sure if I should be using a Result for this
// TODO: At least use proper generics inside the Result
pub fn handle_key(editor: &mut Editor, key: Key) -> ControlCommand {
    let buffer = editor.get_focused_buffer_mut();

    match key {
        Key::Char(c) => {
            buffer.insert_char(c);
            return ControlCommand::Redraw(Redraw::FocusedView);
        }
        Key::Left => {
            buffer.go(TextObject::Char, Direction::Left);
            return ControlCommand::Redraw(Redraw::Cursor);
        }
        Key::Right => {
            buffer.go(TextObject::Char, Direction::Right);
            return ControlCommand::Redraw(Redraw::Cursor);
        }
        Key::Backspace => {
            buffer.delete(TextObject::Char, Direction::Left);
            return ControlCommand::Redraw(Redraw::FocusedView);
        }
        Key::Ctrl(c) => match c {
            'd' => {
                buffer.delete(TextObject::Char, Direction::Right);
                return ControlCommand::Redraw(Redraw::FocusedView);
            }
            'b' => {
                buffer.go(TextObject::Char, Direction::Left);
                return ControlCommand::Redraw(Redraw::Cursor);
            }
            'f' => {
                buffer.go(TextObject::Char, Direction::Right);
                return ControlCommand::Redraw(Redraw::Cursor);
            }
            'n' => {
                buffer.go(TextObject::Line, Direction::Down);
                return ControlCommand::Redraw(Redraw::Cursor);
            }
            'p' => {
                buffer.go(TextObject::Line, Direction::Up);
                return ControlCommand::Redraw(Redraw::Cursor);
            }
            'e' => {
                buffer.go(TextObject::Line, Direction::Right);
                return ControlCommand::Redraw(Redraw::Cursor);
            }
            'a' => {
                buffer.go(TextObject::Line, Direction::Left);
                return ControlCommand::Redraw(Redraw::Cursor);
            }
            's' => {
                editor.save_buffer();
                return ControlCommand::Redraw(Redraw::StatusLine);
            }
            'c' => return ControlCommand::Editor(EditorCommand::Quit),
            _ => {
                todo!("Ctrl-{} not implemented", c);
            }
        }
        Key::Alt(c) => match c {
            'f' => {
                buffer.go(TextObject::Word, Direction::Right);
                return ControlCommand::Redraw(Redraw::Cursor);
            }
            'b' => {
                buffer.go(TextObject::Word, Direction::Left);
                return ControlCommand::Redraw(Redraw::Cursor);
            }
            'd' => {
                buffer.delete(TextObject::Word, Direction::Right);
                return ControlCommand::Redraw(Redraw::Cursor);
            }
            _ => {
                todo!("Meta-{} not implemented", c);
            }
        }
        _ => {
            // todo!("key not handled key: {:?}", key);
        }
    }
    return ControlCommand::Noop;
}
