use std::{env, thread, time};
use rand::prelude::*;

enum directions {
    N,
    E,
    S,
    W,
}

#[derive(Clone, Copy)]
struct Cell {
    visited: bool,
    walls: [bool; 4] // [top, right, bottom, left]
}

impl Cell {
    fn escavate(&mut self, direction: directions) {
        match direction {
            directions::N => {
                self.walls[0] = false;
            }
            directions::E => {
                self.walls[1] = false;
            }
            directions::S => {
                self.walls[2] = false;
            }
            directions::W => {
                self.walls[3] = false;
            }
        }
    }
}

fn carve_path(x: usize, y: usize, grid: &mut Vec<Vec<Cell>>, rng: &mut ThreadRng, size: usize) {
    print_maze(&grid, size);
    let mut directions= [directions::N, directions::E, directions::S, directions::W];
    directions.shuffle(rng);
    grid[y][x].visited = true;
    for i in 0..4 as usize{
        match directions[i] {
            directions::N => {
                if y == 0 {
                    continue;
                } else if !grid[y-1][x].visited {
                    grid[y][x].escavate(directions::N);
                    grid[y - 1][x].escavate(directions::S);
                    carve_path(x, y-1, grid, rng, size);
                }
            }
            directions::E => {
                if x == size - 1 {
                    continue;
                } else if !grid[y][x+1].visited {
                    grid[y][x].escavate(directions::E);
                    grid[y][x+1].escavate(directions::W);
                    carve_path(x+1, y, grid, rng, size);
                }
            }
            directions::S => {
                if y == size - 1 {
                    continue;
                } else if !grid[y+1][x].visited {
                    grid[y][x].escavate(directions::S);
                    grid[y+1][x].escavate(directions::N);
                    carve_path(x, y+1, grid, rng, size);
                }
            }
            directions::W => {
                if x == 0 {
                    continue;
                } else if !grid[y][x-1].visited {
                    grid[y][x].escavate(directions::W);
                    grid[y][x-1].escavate(directions::E);
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
                walls: [true; 4],
            };
            size
        ];
        size
    ];
    
    carve_path(0, 0, &mut maze, &mut rng, size);
}

fn print_maze(grid: &Vec<Vec<Cell>>, size: usize) {
    print!("{}[2J", 27 as char);
    thread::sleep(time::Duration::from_millis(50));
    for y in 0..size {
        // Print top walls
        for x in 0..size {
            print!("{}", if grid[y][x].walls[0] { "+---" } else { "+   " });
        }
        println!("+");

        // Print side walls and spaces
        for x in 0..size {
            print!("{}", if grid[y][x].walls[3] { "|   " } else { "    " });
        }
        println!("|");
    }

    // Bottom line
    for _ in 0..size {
        print!("+---");
    }
    println!("+");
}