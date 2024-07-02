#![feature(ascii_char)]

mod rules;
mod graphics;

use std::io::{Stdin, stdin, Stdout, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};


pub enum Mode {
    PvP,
    AI
}

pub enum GameStatus {
    Started,
    Ended,
    NotStarted
}

pub enum CommandDebug {
    Valid,
    InValid
}

pub enum Round {
    White,
    Black
}

pub struct Game {
    pub(crate) mode: Mode,
    pub(crate) status: GameStatus,
    pub(crate) board: Board,
    pub(crate) cmd_debug: CommandDebug,
    pub(crate) round: Round
}


pub struct Board {
    pub(crate) board: [[usize; 8]; 8],
    pub(crate) white_original_position_checkers: [bool; 3],
    pub(crate) black_original_position_checkers: [bool; 3],
}

impl Game {
    fn new() -> Self {
        Game {
            mode: Mode::PvP,
            status: GameStatus::Started,
            board: Board::new(),
            cmd_debug: CommandDebug::InValid,
            round: Round::White
        }
    }

    fn init(&mut self) {
        let stdin: Stdin = stdin();
        let mut stdout: RawTerminal<Stdout> = stdout().into_raw_mode().unwrap();

        graphics::clear_screen();
        graphics::start_screen();

        // key catching

        for c in stdin.keys() {
            match c.unwrap() {
                Key::Char('q') => break,
                Key::Char('h') => graphics::help_screen(),
                Key::Char('s') => graphics::start_screen(),
                Key::Char('r') => graphics::display_all(self.board.board),
                Key::Char('m') => {
                    match self.mode {
                        Mode::PvP => {
                            if rules::move_piece(self) == true {
                                Game::update(self);
                            }
                        }

                        _ => {}
                    }
                }
                _ => {}
            }
            stdout.flush().unwrap();
        }

        write!(stdout, "{}", termion::cursor::Show).unwrap();
        //Game::update(self);
    }

    fn update(&mut self) {

        match self.round {
            Round::White => {
                self.round = Round::Black;
            }
            Round::Black => {
                self.round = Round::White;
            }
            _ => {}
        }

        graphics::display_all(self.board.board);

    }

}

impl Board {
    fn new() -> Self {
        Board {
            board : [
                [11, 7, 0, 0, 0, 0, 0, 1],
                [7, 12, 0, 6, 0, 6, 6, 6],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 2, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 6, 12, 12, 0, 12, 12, 12],
                [7, 0, 0, 0, 1, 0, 0, 7]
            ],
            white_original_position_checkers : [true, true, true],
            black_original_position_checkers : [true, true, true],
        }
    }
}

fn main() {
    let mut game = Game::new();
    Game::init(&mut game);
}