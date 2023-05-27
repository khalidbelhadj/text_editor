use std::{
    io::{stdin, stdout, Write},
    path::PathBuf,
    process::exit,
    str::FromStr,
};

use clap::Parser;
use termion::{event::Key, input::TermRead};

use crate::{
    buffer::{Buffer, Direction, TextObject},
    cli::CLIArgs,
    editor::Editor,
    renderer::{
        debug_terminal_renderer::DebugTerminalRenderer, terminal_renderer::TerminalRenderer,
        Renderer,
    },
};

#[derive(Clone, Copy)]
pub enum EditorState {
    Editing,
    PromptResponse,
}

pub fn run() {
    let args = CLIArgs::parse();

    let debug = args.debug;
    let path = args
        .path
        .map(|path| PathBuf::from_str(path.as_str()).unwrap());

    let mut renderer: Box<dyn Renderer> = if debug {
        Box::new(DebugTerminalRenderer::new())
    } else {
        Box::new(TerminalRenderer::new())
    };

    let mut editor = Editor::new();
    editor.open_file(path);

    renderer.render(&editor);
    stdout().flush().unwrap();
    let mut it = stdin().keys();

    loop {
        let key = it.next().unwrap().unwrap();
        handle_key(&mut editor, &mut renderer, key);
    }
}

pub fn handle_key(editor: &mut Editor, renderer: &mut Box<dyn Renderer>, key: Key) {
    let buffer: &mut Buffer;
    let state = editor.get_state();

    match state {
        EditorState::Editing => buffer = editor.get_focused_buffer_mut(),
        EditorState::PromptResponse => {
            buffer = editor.get_minibuffer();
        }
    }

    match key {
        Key::Char(c) => {
            buffer.insert(c);
            renderer.render(&editor);
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
                match buffer.path {
                    Some(_) => editor.save_buffer(None),
                    None => {
                        editor.state = EditorState::PromptResponse;
                        match prompt(editor, renderer, "Enter a file name") {
                            Some(new_path) => {
                                editor.save_buffer(Some(new_path));
                            }
                            None => {
                                editor.state = EditorState::Editing;
                            }
                        }
                    }
                }
                renderer.render_status_line(&editor);
                renderer.render_cursor(&editor);
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

fn prompt(editor: &mut Editor, renderer: &mut Box<dyn Renderer>, message: &str) -> Option<String> {
    // TODO: Make this optional so that user can cancel operation
    // TODO: add extra key bindings here instead like \n and C-g
    assert!(matches!(editor.state, EditorState::PromptResponse));

    let mut it = stdin().keys();
    loop {
        renderer.render_minibuffer_prompt(editor, message);
        let key = it.next().unwrap().unwrap();
        match key {
            Key::Char(c) => match c {
                '\n' => {
                    editor.state = EditorState::Editing;
                    break;
                }
                _ => editor.minibuffer.insert(c),
            },
            Key::Ctrl(c) => {
                match c {
                    's' => {
                        // Don't allow saves in minibuffer
                    }
                    'g' => {
                        renderer.clear_minibuffer(&editor);
                        return None;
                    }
                    _ => handle_key(editor, renderer, key),
                }
            }
            _ => handle_key(editor, renderer, key),
        }
    }

    let response = editor.minibuffer.text().iter().collect::<String>();
    editor.minibuffer.clear();
    renderer.clear_minibuffer(&editor);
    Some(response)
}
