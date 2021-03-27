mod utils;

use fixedbitset::FixedBitSet;
use js_sys::Math;
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

const DEAD: bool = false;
const ALIVE: bool = true;

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
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

// impl fmt::Display for Universe {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         for line in self.cells.as_slice().chunks(self.width as usize) {
//             for &cell in line {
//                 let symbol = if cell == DEAD { '◻' } else { '◼' };
//                 write!(f, "{}", symbol)?;
//             }
//             write!(f, "\n")?;
//         }

//         Ok(())
//     }
// }

// Public methods, exported to JavaScript
#[wasm_bindgen]
impl Universe {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
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
                    (ALIVE, x) if x < 2 => DEAD,
                    // Rule 2 (Survive)
                    (ALIVE, 2) | (ALIVE, 3) => ALIVE,
                    // Rule 3 (Overpopulation)
                    (ALIVE, x) if x > 3 => DEAD,
                    // Rule 4 (Reproduction)
                    (DEAD, 3) => ALIVE,
                    // All other cells are unchanged
                    (otherwise, _) => otherwise,
                };

                next.set(idx, next_cell);
            }
        }

        self.cells = next;
    }

    // pub fn new() -> Universe {
    //     let width = 64;
    //     let height = 64;

    //     let cells = (0..width * height)
    //         .map(|i| {
    //             if i % 2 == 0 || i % 7 == 0 {
    //                 ALIVE
    //             } else {
    //                DEAD
    //             }
    //         })
    //         .collect();

    //     Universe {
    //         width,
    //         height,
    //         cells,
    //     }
    // }
    pub fn random() -> Universe {
        let width = 100;
        let height = 100;

        let mut cells = FixedBitSet::with_capacity(width * height);

        for i in 0..width * height {
            cells.set(i, if Math::random() < 0.5 { ALIVE } else { DEAD });
        }

        Universe {
            width: width as u32,
            height: height as u32,
            cells,
        }
    }

    pub fn single_spaceship() -> Universe {
        let width: i32 = 64;
        let height: i32 = 64;

        let mut cells = FixedBitSet::with_capacity((width * height) as usize);

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

        let ship_lines: Vec<&str> = ship.split('\n').collect();
        let ship_width: i32 = ship_lines[0].chars().count().try_into().unwrap();
        let ship_height: i32 = ship_lines.len().try_into().unwrap();

        let ship_start = (width / 2 - ship_width / 2, height / 2 - ship_height / 2);

        for i in 0..width * height {
            let x = i % width;
            let y = i / height;
            let ship_coord: (i32, i32) = (x - ship_start.0, y - ship_start.1);

            let cell = if ship_coord.0 >= 0
                && ship_coord.0 < ship_width
                && ship_coord.1 >= 0
                && ship_coord.1 < ship_height
            {
                if ship_lines[ship_coord.1 as usize]
                    .chars()
                    .nth(ship_coord.0 as usize)
                    == Some('■')
                {
                    ALIVE
                } else {
                    DEAD
                }
            } else {
                DEAD
            };

            cells.set(i as usize, cell);
        }

        Universe {
            width: width as u32,
            height: height as u32,
            cells,
        }
    }
}
