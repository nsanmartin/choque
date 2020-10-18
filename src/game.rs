extern crate wasm_bindgen;
extern crate wasm_timer;
use std::sync::mpsc;

pub mod world;
pub mod cell;
pub mod utils;
pub mod clock;

use wasm_bindgen::prelude::*;
//use wasm_bindgen::JsCast;
use log;

use media;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Down,
    Up
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    Play,
    GameOver
}

#[wasm_bindgen]
pub struct Game {
    board: world::Board,
    media: media::WasmMedia,
    clock: clock::Clock,
    keyboard: Direction, //VecDeque<Direction>
    receiver: mpsc::Receiver<Direction>,
    state: State
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Game {
        let (tx, rx) = mpsc::channel();
        let receiver = rx;
        let board = world::Board::new();
        let media = media::WasmMedia::new(774, 774, tx);
        let clock = clock::Clock::new(60);
        let keyboard = Direction::Down;
        let state = State::Play;
        Game {
            board,
            media,
            clock,
            keyboard,
            receiver,
            state
        }
    }

    pub fn process_input(&mut self) {
        loop {
            let _ = match self.receiver.try_recv() {
                Ok(Direction::Left) => self.arrow_left(),
                Ok(Direction::Right) => self.arrow_right(),
                Ok(Direction::Down) => self.arrow_down(),
                Ok(Direction::Up) => self.arrow_up(),
                _ => { break; }
            };
        }

    }
    pub fn update(&mut self) {

        if self.state == State::Play {
            self.board.tick(self.keyboard);
            if self.board.is_game_over {
                self.state = State::GameOver;
            }
        }
    }
    pub fn render(&self) {
        self.board.draw_cells(&self.media.get_context());
    }

    pub fn run_loop(&mut self) {
        self.process_input();
        self.update();
        self.render();
        self.clock.wait();
    }

    pub fn arrow_left(&mut self) {
        self.keyboard = Direction::Left;
    }
    pub fn arrow_right(&mut self) {
        self.keyboard = Direction::Right;
    }
    pub fn arrow_down(&mut self) {
        self.keyboard = Direction::Down;
    }
    pub fn arrow_up(&mut self) {
        self.keyboard = Direction::Up;
    }
    pub fn is_over(self) -> bool {
        self.state == State::GameOver
    }
}

