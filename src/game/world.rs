extern crate wasm_bindgen;


use crate::game::cell::Cell;
use crate::game::utils;
use crate::game::Direction;
use std::fmt;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

use log;

type Row = usize;
type Col = usize;

pub struct Snake {
    width: usize, // todo: ask univers
    height: usize,
    pub row: usize,
    pub col: usize
    //direction:Direction
}

impl Snake {
    pub fn update(&mut self, direction:Direction) {
        match direction {
            Direction::Left => self.col = (self.width + self.col - 1) % self.width,
            Direction::Right => self.col = (self.col + 1) % self.width,
            Direction::Down => self.row = (self.row + 1) % self.height,
            Direction::Up => self.row = (self.height + self.row - 1) % self.height,
        }
        if self.col > self.width { panic!("{}", "self.col > self.width"); }
        if self.row > self.height { panic!("{}", "self.row > self.height"); }
    }
}


#[wasm_bindgen] //todo:sacar
pub struct Board {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
    moving: Snake,
    pub is_game_over: bool
}

impl Board {
    pub fn get_row(&self, row: usize) -> Row {
        ((row % self.width) + self.width) % self.width
    }

    pub fn get_col(&self, col: usize) -> Col {
        ((col % self.height) + self.height) % self.height
    }

    pub fn get_index(&self, row: usize, column: usize) -> usize {
        (row * self.width + column) as usize
    }
}

#[wasm_bindgen]
impl Board {
    pub fn tick(&mut self, direction:Direction) {
        self.moving.update(direction);
        let moving_index = self.get_index(self.moving.row, self.moving.col);
        if self.cells[moving_index] == Cell::Filled {
            log!("{}", "game over");
            self.is_game_over = true;
        }
        else { self.cells[moving_index] = Cell::Filled; }
    }

    pub fn new() -> Board {
        utils::set_panic_hook();

        let width = 128;
        let height = 128;
        let cells = vec![Cell::Empty; width * height];
        let row = 0;
        let col = width/2;
        let moving = Snake{
            width,
            height,
            row,
            col
        };
        let is_game_over = false;
        
        Board {
            width,
            height,
            cells,
            moving,
            is_game_over
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn set_width(&mut self, width: usize) {
        self.width = width;
        self.cells = (0..width * self.height).map(|_i| Cell::Empty).collect();
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn set_height(&mut self, height: usize) {
        self.height = height;
        self.cells = (0..self.width * height).map(|_i| Cell::Empty).collect();
    }

    pub fn draw_cells(&self, ctx:&CanvasRenderingContext2d) {

        let filled_color = JsValue::from_str("#000000");
        let empty_color = JsValue::from_str("#FFFFFF");

        //todo: parametrizar esto
        let cell_size = 5;

        ctx.begin_path();

        ctx.set_fill_style(&filled_color);
        for row in 0..self.height()  {
            for col in 0..self.width() {
                let idx = self.get_index(row, col);
                if self.cells[idx] ==  Cell::Filled {
                    ctx.fill_rect(
                        (col * (cell_size + 1) + 1) as f64,
                        (row * (cell_size + 1) + 1) as f64,
                        cell_size as f64,
                        cell_size as f64
                    );
                }
            }
        }

        ctx.set_fill_style(&empty_color);
        for row in 0..self.height()  {
            for col in 0..self.width() {
                let idx = self.get_index(row, col);
                if self.cells[idx] == Cell::Empty {
                    ctx.fill_rect(
                        (col * (cell_size + 1) + 1) as f64,
                        (row * (cell_size + 1) + 1) as f64,
                        cell_size as f64,
                        cell_size as f64
                    );
                }
            }
        }

        ctx.stroke();
    }

}


impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Empty { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
