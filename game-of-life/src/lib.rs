mod utils;

use std::convert::TryInto;
use std::fmt;
use wasm_bindgen::prelude::*;
use web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)] // Represent each cell as a single byte
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter() {
            for delta_col in [self.width - 1, 0, 1].iter() {
                if *delta_row == 0 && *delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

// Public methods, exported to JavaScript
#[wasm_bindgen]
impl Universe {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1 (Underpopulation)
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2 (Survive)
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3 (Overpopulation)
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule 4 (Reproduction)
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other cells are unchanged
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }

    pub fn new() -> Universe {
        let width = 64;
        let height = 64;

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }
    pub fn single_spaceship() -> Universe {
        let width = 64;
        let height = 64;

        let ship = "___■■__■■___\n\
                    _____■■_____\n\
                    _____■■_____\n\
                    __■_■__■_■__\n\
                    __■______■__\n\
                    ____________\n\
                    __■______■__\n\
                    ___■■__■■___\n\
                    ____■■■■____\n\
                    ____________\n\
                    _____■■_____\n\
                    _____■■_____";

        // - Calculate ship width and height
        let ship_lines: Vec<&str> = ship.split('\n').collect();
        let ship_width: i32 = ship_lines[0].chars().count().try_into().unwrap();
        let ship_height: i32 = ship_lines.len().try_into().unwrap();

        log!("shipLines {:?}", ship_lines);
        log!("shipWidth {:?}", ship_width);
        log!("shipHeight {:?}", ship_height);
        // - Start ship at width/2-shipWidth/2, height/2-shipHeight/2
        let ship_start = (width / 2 - ship_width / 2, height / 2 - ship_height / 2);
        // - Coord to ship cord? x: x-shipStart.x, y: y-shipStart.y
        // - Set cell to alive or dead

        let cells = (0..width * height)
            .map(|i| {
                let x = i % width;
                let y = i / height;
                // log!("x {}, y {}", x, y);
                let ship_coord: (i32, i32) = (x - ship_start.0, y - ship_start.1);
                if ship_coord.0 >= 0
                    && ship_coord.0 < ship_width
                    && ship_coord.1 >= 0
                    && ship_coord.1 < ship_height
                {
                    log!(
                        "{:?}, {:?}",
                        ship_lines[ship_coord.1 as usize]
                            .chars()
                            .nth(ship_coord.0 as usize),
                        ship_coord
                    );
                    if ship_lines[ship_coord.1 as usize]
                        .chars()
                        .nth(ship_coord.0 as usize)
                        == Some('■')
                    {
                        Cell::Alive
                    } else {
                        Cell::Dead
                    }
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width: width as u32,
            height: height as u32,
            cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}
