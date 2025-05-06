use std::{env, thread, time};
use rand::prelude::*;
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

enum Directions {
    N,
    E,
    S,
    W,
}

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

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

fn main() -> Result<(), String>{
    let args: Vec<String> = env::args().collect();
    let size = args[1].parse::<usize>().unwrap();
    let mut rng = rand::rng();
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    
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

    let window = video_subsystem
            .window("rust maze generator", WINDOW_WIDTH, WINDOW_HEIGHT)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..}
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        canvas.clear();
        canvas.present();
        carve_path(0, 0, &mut maze, &mut rng, size);
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }
    
    Ok(())
}

fn print_maze(grid: &Vec<Vec<Cell>>, size: usize) {
    print!("{}[2J", 27 as char);
    thread::sleep(time::Duration::from_millis(50));
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