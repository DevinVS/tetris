use sdl2::pixels::Color;
use sdl2::EventPump;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::rect::Rect;

use tetris::tetromino::Tetromino;
use tetris::TetrisMap;


struct TetrisEngine {
    running: bool,
    event_loop: EventPump,
    canvas: Canvas<Window>,
    tetris_map: TetrisMap,
    live_tetromino: Tetromino
}

impl TetrisEngine {
    fn new(event_loop: EventPump, canvas: Canvas<Window>) -> Self {
        Self {
            running: true,
            event_loop,
            canvas,
            tetris_map: tetris::blank_tetris_map(),
            live_tetromino: Tetromino::new()
        }
    }

    fn delete_full_rows(&mut self) {
        let mut dropped = -1;

        for row in 0..self.tetris_map.len()-1 {
            if self.tetris_map[row].iter().all(|x| x!=&0) {
                dropped = row as isize;
                self.tetris_map[row] = [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
            }
        }

        if dropped != -1 {

            // Iterate over the rows above the deleted row
            for row in (0..dropped as usize).rev() {
                let mut contiguous_cells = Vec::<usize>::new();
                let mut group = false;

                for col in 0..self.tetris_map[0].len() {
                    if self.tetris_map[row][col] > 1 {
                        group = true;
                        contiguous_cells.push(col);
                    } else if group {
                        self.drop_group(row, &contiguous_cells);
                        contiguous_cells = Vec::<usize>::new();
                        group = false;
                    }
                }
            }
        }
    }

    fn drop_group(&mut self, mut row: usize, cols: &Vec<usize>) {
        while cols.iter()
            .map(|col| self.tetris_map[row+1][*col])
            .all(|x| x==0) {

            for col in cols {
                self.tetris_map[row+1][*col] = self.tetris_map[row][*col];
                self.tetris_map[row][*col] = 0;
            }
            row += 1;
        }
    }
}


fn main() {
    // Set up game globals
    let sdl_context = sdl2::init().unwrap();
    let video_subsytem = sdl_context.video().unwrap();

    let window = video_subsytem.window("tetris", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();

    // Event Loop
    let mut event_loop = sdl_context.event_pump().unwrap();

    // Engine Struct
    let mut engine = TetrisEngine::new(event_loop, canvas);

    let mut start_time = std::time::Instant::now();

    while engine.running {

        handle_events(&mut engine);

        if std::time::Instant::now().duration_since(start_time).as_secs_f32() > 0.35 {
            update(&mut engine);
            start_time = std::time::Instant::now();
        }

        draw(&mut engine);

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn handle_events(engine: &mut TetrisEngine) {
    for event in engine.event_loop.poll_event() {
        match event {
            Event::Quit {..} => engine.running = false,
            Event::KeyDown {keycode: Some(keycode), ..} => {
                match keycode {
                    Keycode::Left => engine.live_tetromino.left(&engine.tetris_map),
                    Keycode::Right => engine.live_tetromino.right(&engine.tetris_map),
                    Keycode::Down => {
                        while engine.live_tetromino.down(&engine.tetris_map) {}
                    },
                    Keycode::Up => {
                        engine.live_tetromino.rotate(&engine.tetris_map);
                    }
                    _ => {}
                }
            },
            _ => {}
        }
    }
}

fn update(engine: &mut TetrisEngine) {
    let moved = engine.live_tetromino.down(&engine.tetris_map);

    if !moved {
        if engine.live_tetromino.y == 0 {
            engine.running = false;
        } else {
            engine.live_tetromino.add_to_map(&mut engine.tetris_map);
            engine.delete_full_rows();
            engine.live_tetromino = Tetromino::new();
        }
    }
}


fn draw(engine: &mut TetrisEngine) {
    engine.canvas.set_draw_color(Color::RGB(255, 255, 255));
    engine.canvas.clear();

    let size: u32= 20;

    let mut tetris_map = engine.tetris_map;
    engine.live_tetromino.add_to_map(&mut tetris_map);

    for row in 0..23 {
        for col in 0..12 {
            let color_code = tetris_map[row][col];
            engine.canvas.set_draw_color(color_code_to_color(color_code));

            engine.canvas.fill_rect(
                Rect::new(col as i32 * size as i32 + 280, row as i32*size as i32, size, size)
            ).unwrap();
        }
    }

    engine.canvas.present()
}

fn color_code_to_color(color_code: u8) -> Color {
    match color_code {
        0 => Color::WHITE,
        1 => Color::BLACK,
        2 => Color::RGB(135, 251, 255),
        3 => Color::RGB(79, 108, 255),
        4 => Color::RGB(255, 193, 79),
        5 => Color::RGB(245, 245, 100),
        6 => Color::RGB(122, 245,100),
        7 => Color::RGB(182, 100, 245),
        8 => Color::RGB(245, 112, 100),
        _ => Color::WHITE
    }
}
