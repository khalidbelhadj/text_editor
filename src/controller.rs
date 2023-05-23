use std::{
    env::args,
    io::{stdin, stdout, Write},
    path::PathBuf,
    str::FromStr,
};

use termion::{
    event::Key,
    input::TermRead
};

use crate::{
    editor::Editor,
    renderer::{
        Renderer,
        TerminalRenderer
    },
};

pub enum EditorCommand {
    ToggleDebug,
    Quit,
}

pub enum RedrawItem {
    All,
    Cursor,
    FocusedView,
    StatusLine,
}

pub enum ControlCommand {
    Redraw(RedrawItem),
    // QueKey(Key),
    Editor(EditorCommand),
    Noop,
}

// (path, debug)
type CLIArgs = (Option<PathBuf>, bool);

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
    (path, debug)
}

pub fn run() {
    let (path, debug) = get_cli_args();

    // let stdin = termion::async_stdin();
    let stdin = stdin();

    let mut editor = Editor::new();
    let mut renderer = TerminalRenderer::new();
    renderer.debug = debug;
    editor.open_file(path);

    renderer.render(&editor);
    stdout().flush().unwrap();
    let mut it = stdin.keys();

    loop {
        let res = it.next().expect("Something went wrong");
        let key = res.expect("Not sure what happened but stdin was invalid for some reason");

        match editor.handle_key(key) {
            ControlCommand::Editor(command) => match command {
                EditorCommand::ToggleDebug => {
                    renderer.debug = !renderer.debug;
                }
                EditorCommand::Quit => {
                    return;
                }
            },
            ControlCommand::Redraw(command) => match command {
                RedrawItem::All => {
                    renderer.render(&editor);
                }
                RedrawItem::Cursor => {
                    renderer.render_status_line(&editor);
                    renderer.render_cursor(&editor);
                }
                RedrawItem::FocusedView => {
                    renderer.render(&editor);
                }
                RedrawItem::StatusLine => {
                    renderer.render_status_line(&editor);
                }
            },
            ControlCommand::Noop => {}
        }
        stdout().flush().unwrap();
    }
}
