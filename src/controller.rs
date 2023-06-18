use std::{
    io::stdin,
    path::PathBuf,
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
    Selecting,
}

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {}] {}",
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}

pub fn run() {
    setup_logger().unwrap();

    // Parsing CLI args
    let args = CLIArgs::parse();

    let debug = args.debug;
    let path = args
        .path
        .map(|path| PathBuf::from_str(path.as_str()).unwrap());

    // Setting up renderer and editor
    let mut renderer: Box<dyn Renderer> = if debug {
        Box::new(DebugTerminalRenderer::new())
    } else {
        Box::new(TerminalRenderer::new())
    };

    let mut editor = Editor::new();
    editor.open_file(path);

    // Main Loop
    renderer.render_all(&editor);
    let mut it = stdin().keys();

    loop {
        let key = it.next().unwrap().unwrap();
        handle_key(&mut editor, &mut renderer, key);
    }
}

pub fn handle_key(editor: &mut Editor, renderer: &mut Box<dyn Renderer>, key: Key) {
    let buffer: &mut Buffer;
    let state = &mut editor.state;

    match state {
        EditorState::PromptResponse => buffer = &mut editor.minibuffer,
        _ => buffer = editor.get_focused_buffer_mut(),
    }

    match key {
        Key::Char(c) => {
            buffer.insert(c);
            match c {
                '\n' => renderer.render_all(&editor),
                _ => {
                    renderer.render_status_line(&editor);
                    renderer.render_line(&editor)
                },
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
        Key::Up => {
            buffer.go(TextObject::Line, Direction::Up);
            renderer.render_status_line(&editor);
            renderer.render_cursor(&editor);
        }
        Key::Down => {
            buffer.go(TextObject::Line, Direction::Down);
            renderer.render_status_line(&editor);
            renderer.render_cursor(&editor);
        }
        Key::Backspace => {
            let old_line_count = buffer.line_count();
            buffer.delete(TextObject::Char, Direction::Left);

            if old_line_count == buffer.line_count() {
                renderer.render_status_line(&editor);
                renderer.render_line(&editor);
            } else {
                renderer.render_all(&editor);
            }
        }
        Key::Ctrl(c) => match c {
            'd' => {
                let old_line_count = buffer.line_count();
                buffer.delete(TextObject::Char, Direction::Right);

                if old_line_count == buffer.line_count() {
                    renderer.render_status_line(&editor);
                    renderer.render_line(&editor);
                } else {
                    renderer.render_all(&editor);
                }
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
            'k' => {
                buffer.delete(TextObject::Line, Direction::Right);
                renderer.render_all(&editor);
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
            't' => {
                buffer.toggle_selection();
                editor.state = EditorState::Selecting;
                handle_key_selection(editor, renderer);
            }
            'c' => {
                panic!("not sure how to implement exit")
            }
            'y' => {
                buffer.paste_from_clipboard();
                renderer.render_all(&editor);
            }
            _ => {
                todo!("Ctrl-{} not implemented", c);
            }
        },
        Key::Alt(c) => match c {
            'w' => {
                buffer.copy_to_clipboard();
            }
            'f' => {
                buffer.go(TextObject::Word, Direction::Right);
                renderer.render_status_line(&editor);
                renderer.render_cursor(&editor);
            }
            'b' => {
                buffer.go(TextObject::Word, Direction::Left);
                renderer.render_status_line(&editor);
                renderer.render_cursor(&editor);
            }
            'd' => {
                let old_line_count = buffer.line_count();
                buffer.delete(TextObject::Word, Direction::Right);

                if old_line_count == buffer.line_count() {
                    renderer.render_status_line(&editor);
                    renderer.render_line(&editor);
                } else {
                    renderer.render_all(&editor);
                }
            }
            '\u{7f}' => {
                let old_line_count = buffer.line_count();
                buffer.delete(TextObject::Word, Direction::Left);

                if old_line_count == buffer.line_count() {
                    renderer.render_status_line(&editor);
                    renderer.render_line(&editor);
                } else {
                    renderer.render_all(&editor);
                }
            }
            '<' => {
                buffer.go_to_start();
                renderer.render_all(&editor);
            }
            '>' => {
                buffer.go_to_end();
                renderer.render_all(&editor);
            }
            _ => {}
        },
        _ => {}
    }
}

fn prompt(editor: &mut Editor, renderer: &mut Box<dyn Renderer>, message: &str) -> Option<String> {
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

fn handle_key_selection(editor: &mut Editor, renderer: &mut Box<dyn Renderer>) {
    assert!(matches!(editor.state, EditorState::Selecting));

    let mut it = stdin().keys();
    loop {
        renderer.render_all(editor);
        let key = it.next().unwrap().unwrap();

        match key {
            Key::Char(_) => {
                handle_key(editor, renderer, key);
                editor.get_focused_buffer_mut().delete_selection();
                renderer.render_all(editor);
                break;
            }
            Key::Backspace => {
                editor.get_focused_buffer_mut().delete_selection();
                renderer.render_all(editor);
                break;
            }
            Key::Ctrl(c) => {
                match c {
                    't' | 'g' => {
                        editor.get_focused_buffer_mut().toggle_selection();
                        renderer.render_all(editor);
                        break;
                    }
                    _ => {
                        handle_key(editor, renderer, key);
                    }
                }
            }
            _ => handle_key(editor, renderer, key),
        }
    }
    editor.state = EditorState::Editing;
}
