#![allow(unused)]
#![allow(dead_code)]

use std::env::args;

mod buffer;
use buffer::{Buffer, View};

mod renderer;
use renderer::{NCursesRenderer, Renderer};

use termion::{event::Key, raw::RawTerminal};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use std::io::{Write, stdout, stdin, Stdout};

use termion::{color, clear, style, cursor};

const DEBUG: bool = false;

fn main2() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdin = termion::async_stdin();

    let mut it = stdin.keys();
    let mut buffer: Vec<char> = vec![];

    loop {
        let res = it.next();
        match res {
            Some(key) => {
                match key {
                    Ok(k) => {
                        match k {
                            Key::Char(c) => {
                                buffer.push(c);
                            },
                            _ => todo!("not handled")
                        }
                    },
                    Err(c) => {}
                }
                // draw_buffer(&buffer, &mut stdout);
                stdout.flush().unwrap();
            },
            None => {
            }
        }
    }
}

fn draw_view(view: &View, stdout: &mut RawTerminal<Stdout>) {
    write!(stdout, "{}", termion::clear::All);

    let mut text = &view.text();
    let mut cursor_offset = view.cursor_offset();

    if DEBUG {
        text = &view.buffer.data;
    }

    if DEBUG {
        cursor_offset = view.buffer.cursor_offset;
    }

    let mut line_number: u16 = 1;
    let mut tmp = 0;

    for i in 0..text.len() {
        if text[i] == '\n' {
            line_number += 1;
            tmp = i + 1;
            continue;
        }

        if i == cursor_offset {
            write!(stdout, "{}{}", termion::cursor::Goto((i - tmp + 1) as u16, line_number), '|');
            continue;
        }

        if DEBUG {
            if view.buffer.gap_start <= i && i < view.buffer.gap_start + view.buffer.gap_len {
                write!(stdout, "{}{}", termion::cursor::Goto((i - tmp + 1) as u16, line_number), '_');
                continue;
            }

        }

        write!(stdout, "{}{}", termion::cursor::Goto((i - tmp + 1) as u16, line_number), text[i]);
    }

    if DEBUG {
        write!(stdout, "{}{}", termion::cursor::Goto(1 as u16, line_number + 1), view.buffer.line_offsets.len());
    }

    if cursor_offset == text.len() {
        write!(stdout, "{}{}", termion::cursor::Goto((text.len() - tmp + 1) as u16, line_number), '|');
    }

    // write!(stdout, "{}", termion::cursor::Goto((cursor_offset - tmp + 1) as u16, line_number));
}

fn main() {
    let mut view = View::new(Buffer::new(None), 1, 1, 50, 50);

    let mut stdout = stdout().into_raw_mode().unwrap();
    // let stdin = termion::async_stdin();
    let stdin = stdin();

    write!(stdout, "{}", termion::clear::All).unwrap();
    stdout.flush().unwrap();

    let mut it = stdin.keys();
    let mut buffer: Vec<char> = vec![];

    loop {
        let res = it.next();

        if let Some(key) = res {
            if let Ok(k) = key {
                match k {
                    Key::Char(c) => {
                        view.buffer.insert_char(c);
                    },
                    Key::Left => {
                        view.buffer.move_cursor(-1);
                    },
                    Key::Right => {
                        view.buffer.move_cursor(1);
                    },
                    Key::Backspace => {
                        view.buffer.delete_char_backward();
                    },
                    Key::Ctrl(c) => {
                        match c {
                            'c' => {
                                break;
                            },
                            'd' => {
                                view.buffer.delete_char_forward();
                            },
                            'b' => {
                                view.buffer.move_cursor(-1);
                            },
                            'f' => {
                                view.buffer.move_cursor(1);
                            },
                            'e' => {
                                view.buffer.move_to_eol();
                            },
                            'b' => {
                                view.buffer.move_to_eol();
                            },
                            _ => todo!("Ctrl modifier not implemented")
                        }
                    },
                    _ => {
                        todo!("key not handled key: {:?}", k);
                    }
                }
            } else {
                panic!("not sure what happened but stdin was invalid for some reason");
            }
            draw_view(&view, &mut stdout);
            stdout.flush().unwrap();

        } else {
        }
    }

    write!(stdout, "{}", termion::clear::All).unwrap();
    stdout.flush().unwrap();
}
