use std::{
    env::args,
    fmt::Debug,
    io::{stdin, stdout, Stdin, Write},
    path::PathBuf,
    str::FromStr, process::exit,
};

use termion::{event::Key, input::TermRead};

use crate::{
    buffer::{Direction, TextObject, Buffer},
    editor::Editor,
    renderer::{
        debug_terminal_renderer::DebugTerminalRenderer, terminal_renderer::TerminalRenderer,
        Renderer, self,
    },
};

#[derive(Clone, Copy)]
pub enum EditorState {
    Editing,
    PromptResponse,
}

// (path, debug)
type CLIArgs = (Option<PathBuf>, bool);

pub fn run() {
    let (path, debug) = get_cli_args();

    let mut editor = Editor::new();
    editor.open_file(path);

    // TODO: change this if debug is true
    let mut renderer = TerminalRenderer::new();
    // let mut renderer = DebugTerminalRenderer::new();

    renderer.render(&editor);
    stdout().flush().unwrap();
    let mut it = stdin().keys();

    loop {
        let key = it.next().unwrap().unwrap();
        handle_key(&mut editor, &mut renderer, key);
        stdout().flush().unwrap();
    }
}

pub fn handle_key<T: Renderer>(editor: &mut Editor, renderer: &mut T, key: Key) {
    let buffer: &mut Buffer;
    let state = editor.get_state();

    match state {
        EditorState::Editing => {
            buffer = editor.get_focused_buffer_mut()
        }
        EditorState::PromptResponse => {
            buffer = editor.get_minibuffer();
        }
    }


    match key {
        Key::Char(c) => {
            match state {
                EditorState::Editing => {
                    buffer.insert(c);
                    renderer.render(&editor);
                }
                EditorState::PromptResponse => {
                    match c {
                        '\n' => {
                            editor.state = EditorState::Editing;
                        }
                        _ => {
                            buffer.insert(c);
                        }
                    }
                }
            }
        }
        Key::Left => {
            buffer.go(TextObject::Char, Direction::Left);
            renderer.render_status_line(&editor);
            renderer.render_cursor(&editor);
        }
        Key::Right => {
            buffer.go(TextObject::Char, Direction::Right);
            renderer.render_status_line(&editor);
            renderer.render_cursor(&editor);
        }
        Key::Backspace => {
            buffer.delete(TextObject::Char, Direction::Left);
            renderer.render(&editor);
        }
        Key::Ctrl(c) => match c {
            'd' => {
                buffer.delete(TextObject::Char, Direction::Right);
                renderer.render(&editor);
            }
           'b' => {
                buffer.go(TextObject::Char, Direction::Left);
                renderer.render_status_line(&editor);
                renderer.render_cursor(&editor);
            }
            'f' => {
                buffer.go(TextObject::Char, Direction::Right);
                renderer.render_status_line(&editor);
                renderer.render_cursor(&editor);
            }
            'n' => {
                buffer.go(TextObject::Line, Direction::Down);
                renderer.render_status_line(&editor);
                renderer.render_cursor(&editor);
            }
            'p' => {
                buffer.go(TextObject::Line, Direction::Up);
                renderer.render_status_line(&editor);
                renderer.render_cursor(&editor);
            }
            'e' => {
                buffer.go(TextObject::Line, Direction::Right);
                renderer.render_status_line(&editor);
                renderer.render_cursor(&editor);
            }
            'a' => {
                buffer.go(TextObject::Line, Direction::Left);
                renderer.render_status_line(&editor);
                renderer.render_cursor(&editor);
            }
            's' => {
                if buffer.path.is_none() {
                    editor.state = EditorState::PromptResponse;
                    let new_path = prompt(editor, renderer, "Enter a file name");
                    editor.save_buffer(Some(new_path));
                } else {
                    editor.save_buffer(None);
                    renderer.render_status_line(&editor);
                    renderer.render_cursor(&editor);
                }
            }
            'c' => {
                exit(0);
            }
            _ => {
                todo!("Ctrl-{} not implemented", c);
            }
        },
        Key::Alt(c) => match c {
            'f' => {
                buffer.go(TextObject::Word, Direction::Right);
                renderer.render_cursor(&editor);
            }
            'b' => {
                buffer.go(TextObject::Word, Direction::Left);
                renderer.render_cursor(&editor);
            }
            'd' => {
                buffer.delete(TextObject::Word, Direction::Right);
                renderer.render_cursor(&editor);
            }
            _ => {
                todo!("Meta-{} not implemented", c);
            }
        },
        _ => {
            // todo!("key not handled key: {:?}", key);
        }
    }
}

fn prompt<T: Renderer>(editor: &mut Editor, renderer: &mut T, message: &str) -> String {
    // TODO: Make this optional so that user can cancel operation
    // TODO: add extra key bindings here instead like \n and C-g
    assert!(matches!(editor.state, EditorState::PromptResponse));

    let mut it = stdin().keys();
    loop {
        renderer.render_minibuffer_prompt(editor, message);
        let key = it.next().unwrap().unwrap();
        handle_key(editor, renderer, key);
        if matches!(editor.state, EditorState::Editing) { break; }
    }

    let response = editor.minibuffer.text().iter().collect::<String>();
    editor.minibuffer.clear();
    renderer.clear_minibuffer(&editor);
    renderer.render_cursor(&editor);
    renderer.render_status_line(&editor);
    response
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
