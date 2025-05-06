use std::{env, thread, time};
use rand::prelude::*;

enum Directions {
    N,
    E,
    S,
    W,
}

#[derive(Clone, Copy)]
struct Cell {
    visited: bool,
    walls: u8, // bit 0 = top, bit 1 = left
}

fn escavate(x: usize, y: usize, grid: &mut Vec<Vec<Cell>>, direction: Directions) {
    match direction {
        Directions::N => {
            let current = &mut grid[y][x];
            current.walls &= 0x02; // 00000010
        }
        Directions::E => {
            let neighbor = &mut grid[y][x+1];
            neighbor.walls &= 0x01; // 00000001
        }
        Directions::S => {
            let neighbor = &mut grid[y+1][x];
            neighbor.walls &= 0x02;
        }
        Directions::W => {
            let current = &mut grid[y][x];
            current.walls &= 0x01;
        }
    }
}

fn carve_path(x: usize, y: usize, grid: &mut Vec<Vec<Cell>>, rng: &mut ThreadRng, size: usize) {
    print_maze(&grid, size);
    let mut directions= [Directions::N, Directions::E, Directions::S, Directions::W];
    directions.shuffle(rng);
    grid[y][x].visited = true;
    for i in 0..4 as usize{
        match directions[i] {
            Directions::N => {
                if y == 0 {
                    continue;
                } else if !grid[y-1][x].visited {
                    escavate(x, y, grid, Directions::N);
                    carve_path(x, y-1, grid, rng, size);
                }
            }
            Directions::E => {
                if x == size - 1 {
                    continue;
                } else if !grid[y][x+1].visited {
                    escavate(x, y, grid, Directions::E);
                    carve_path(x+1, y, grid, rng, size);
                }
            }
            Directions::S => {
                if y == size - 1 {
                    continue;
                } else if !grid[y+1][x].visited {
                    escavate(x, y, grid, Directions::S);
                    carve_path(x, y+1, grid, rng, size);
                }
            }
            Directions::W => {
                if x == 0 {
                    continue;
                } else if !grid[y][x-1].visited {
                    escavate(x, y, grid, Directions::W);
                    carve_path(x-1, y, grid, rng, size);
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let size = args[1].parse::<usize>().unwrap();
    let mut rng = rand::rng();
    
    let mut maze = vec![
        vec![
            Cell {
                visited: false,
                walls: 0x03, // 00000011
            };
            size
        ];
        size
    ];
    
    carve_path(0, 0, &mut maze, &mut rng, size);
}

fn print_maze(grid: &Vec<Vec<Cell>>, size: usize) {
    print!("{}[2J", 27 as char);
    thread::sleep(time::Duration::from_millis(200));
    for y in 0..size {
        // Print top walls
        for x in 0..size {
            print!("{}", if grid[y][x].walls & 0x01 != 0 { "+---" } else { "+   " });
        }
        println!("+");

        // Print side walls and spaces
        for x in 0..size {
            print!("{}", if grid[y][x].walls & 0x02 != 0 { "|   " } else { "    " });
        }
        println!("|");
    }

    // Bottom line
    for _ in 0..size {
        print!("+---");
    }
    println!("+");
}