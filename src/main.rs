/* TODO: 
- Zoom to Center
	- currently zooms to top left block
- Implement a UI
	- display basic help info
	- add an FPS counter (current one is very bad)
- Separate Cell Buffer from UI Buffer
	- use separate textures for cells grid and UI
*/

extern crate rand;
extern crate sdl2;
extern crate time;

use rand::Rng;

use sdl2::rect::{Point, Rect};
use sdl2::event::Event;
use sdl2::pixels;
use sdl2::keyboard::Keycode;
use sdl2::render::BlendMode::*;
use sdl2::mouse::MouseButton;
use sdl2::sys::video::SDL_GL_GetDrawableSize;

use time::PreciseTime;

const SCREEN_WIDTH: u32 = 400;
const SCREEN_HEIGHT: u32 = 400;

fn main() {

	// Makes Window
	let sdl_context = sdl2::init().unwrap();
	let video_subsys = sdl_context.video().unwrap();
	let window = video_subsys.window("Game of Life", SCREEN_WIDTH, SCREEN_HEIGHT)
		.position_centered()
		.resizable()
		.opengl()
		.build()
		.unwrap();

	let mut canvas = window.into_canvas().present_vsync().build().unwrap();
	let mut events = sdl_context.event_pump().unwrap();

	canvas.set_draw_color(pixels::Color::RGB(255, 255, 255));
	canvas.clear();
	canvas.present();

	let height:i32 = 600; //How tall
	let width:i32 = 800; //How wide

	let mut scale:i32 = 1; //Sets side length of cells
	let mut x_off:i32 = 0;
	let mut y_off:i32 = 0;

	// Sets up GOL logic
	let mut grid: Vec<bool> = Vec::new();
	// Make grid random
	let mut rand = rand::thread_rng();
	for _ in 0..(height*width) {
		grid.push(rand.gen::<bool>());
	}

	let mut play = true;
	let mut relative_mouse;
	let mut mouse;
	let mut pressed = false;
	let mut time = PreciseTime::now();;

	canvas.set_blend_mode(Blend);

	// Main Loop
	'main: loop {
		
		canvas.set_draw_color(pixels::Color::RGB(100, 100, 100));
		canvas.clear();

		relative_mouse = events.relative_mouse_state();
		mouse = events.mouse_state();

		// Recieves Input Events
		for event in events.poll_iter() {
			match event {
				Event::Quit {..} => break 'main,
				Event::KeyDown { keycode: Some(Keycode::Escape), repeat: false, .. } => {
					break 'main;
				},
				Event::KeyDown { keycode: Some(Keycode::Space), repeat: false, .. } => {
					play = !play;
				},
				Event::MouseWheel { y,.. } => {
					let c_scale = scale;
					scale += y;
					if scale < 1 {
						scale = 1;
					} else if scale > 64 { // Arbitrary limit, consider changing
						scale = 64;
					}
					x_off = x_off * scale / c_scale;
					y_off = y_off * scale / c_scale;
					
				},
				Event::KeyDown { keycode: Some(Keycode::Return), repeat: false, .. } => {
					for a in 0..grid.len()-1 {
						grid[a] = false;
					}
				},
				Event::KeyDown { keycode: Some(Keycode::F), repeat: false, .. } => {
					grid = next_life(&grid, height, width);
					play = false;
				},
				_ => {}
			}
		}

		if events.mouse_state().is_mouse_button_pressed(MouseButton::Right) {
			x_off -= relative_mouse.x();
			y_off -= relative_mouse.y();
		}

		let cursor_x = (mouse.x()+x_off)/scale;
		let cursor_y = (mouse.y()+y_off)/scale;

		if events.mouse_state().is_mouse_button_pressed(MouseButton::Left) {
			if !pressed {
				// Togge Status of Cell;
				if cursor_x >= 0 && cursor_x <= width && cursor_y >= 0 && cursor_y < height {
					grid[(cursor_y*width+cursor_x) as usize] = !grid[(cursor_y*width+cursor_x) as usize];
				}
				pressed = true;
			}
		} else {
			pressed = false;
		}

		canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
		canvas.fill_rect(Rect::new(scale-x_off-scale, scale-y_off-scale, (scale*width) as u32, (scale*height) as u32));

		// Renders Each Frame
		canvas.set_draw_color(pixels::Color::RGBA(255, 255, 255, 255));
		for y in 0..height {
			for x in 0..width {
				// Work on making it only render what will be shown
				if grid[(y*width+x) as usize] {
					canvas.fill_rect(Rect::new(x*scale-x_off, y*scale-y_off, scale as u32, scale as u32));
				}
			}
		}

		canvas.set_draw_color(pixels::Color::RGBA(255, 0, 0, 50));
		canvas.fill_rect(Rect::new(cursor_x*scale-x_off, cursor_y*scale-y_off, scale as u32, scale as u32));

		if play {
			grid = next_life(&grid, height, width);
		}

		

		let frame_time:u32 = 1000 / time.to(PreciseTime::now()).num_milliseconds() as u32;

		time = PreciseTime::now();

		// Draw Frame Rate

		let symbols: [[bool;18];10] = [
			[false,true,false,true,false,true,true,false,true,true,false,true,true,false,true,false,true,false],
			[false,false,true,false,false,true,false,false,true,false,false,true,false,false,true,false,false,true],
			[false,true,false,true,false,true,false,false,true,false,true,false,true,false,false,true,true,true],
			[false,true,false,true,false,true,false,false,true,false,true,false,false,false,true,true,true,false],
			[true,false,true,true,false,true,true,false,true,true,true,true,false,false,true,false,false,true],
			[true,true,true,true,false,false,true,true,false,false,false,true,false,false,true,true,true,false],
			[false,true,true,true,false,false,true,true,false,true,false,true,true,false,true,false,true,false],
			[true,true,true,false,false,true,false,false,true,false,false,true,false,false,true,false,false,true],
			[false,true,false,true,false,true,true,false,true,false,true,false,true,false,true,false,true,false],
			[false,true,false,true,false,true,true,false,true,false,true,true,false,false,true,true,true,false]
		];
		canvas.set_draw_color(pixels::Color::RGBA(255, 0, 0, 255));

		let mut e = frame_time as f64;

		while e > 1.0 {
			e /= 10.0;
		}

		let num_scale = 2;
		let space_scale = 3;
		let mut place = 0;

		for i in 0..frame_time.to_string().len() {
			e = e * 10.0 - (e as i32 * 10) as f64;
			let f = e as i32;
			match f {
				1 => {
					for a in 0..18 as i32 {
						if(symbols[e as usize][a as usize]) {
							canvas.fill_rect(Rect::new((a%3)*space_scale + place, a/3*space_scale, num_scale as u32, num_scale as u32));
						}						
					}
				},
				2 => {
					for a in 0..18 as i32 {
						if(symbols[e as usize][a as usize]) {
							canvas.fill_rect(Rect::new((a%3)*space_scale + place, a/3*space_scale, num_scale as u32, num_scale as u32));
						}
					}

				},
				3 => {
					for a in 0..18 as i32 {
						if(symbols[e as usize][a as usize]) {
							canvas.fill_rect(Rect::new((a%3)*space_scale + place, a/3*space_scale, num_scale as u32, num_scale as u32));
						}
					}
				},
				4 => {
					for a in 0..18 as i32 {
						if(symbols[e as usize][a as usize]) {
							canvas.fill_rect(Rect::new((a%3)*space_scale + place, a/3*space_scale, num_scale as u32, num_scale as u32));
						}
					}

				},
				5 => {
					for a in 0..18 as i32 {
						if(symbols[e as usize][a as usize]) {
							canvas.fill_rect(Rect::new((a%3)*space_scale + place, a/3*space_scale, num_scale as u32, num_scale as u32));
						}
					}

				},
				6 => {
					for a in 0..18 as i32 {
						if(symbols[e as usize][a as usize]) {
							canvas.fill_rect(Rect::new((a%3)*space_scale + place, a/3*space_scale, num_scale as u32, num_scale as u32));
						}
					}

				},
				7 => {
					for a in 0..18 as i32 {
						if(symbols[e as usize][a as usize]) {
							canvas.fill_rect(Rect::new((a%3)*space_scale + place, a/3*space_scale, num_scale as u32, num_scale as u32));
						}
					}

				},
				8 => {
					for a in 0..18 as i32 {
						if(symbols[e as usize][a as usize]) {
							canvas.fill_rect(Rect::new((a%3)*space_scale + place, a/3*space_scale, num_scale as u32, num_scale as u32));
						}
					}

				},
				9 => {
					for a in 0..18 as i32 {
						if(symbols[e as usize][a as usize]) {
							canvas.fill_rect(Rect::new((a%3)*space_scale + place, a/3*space_scale, num_scale as u32, num_scale as u32));
						}
					}

				},
				0 => {
					for a in 0..18 as i32 {
						if(symbols[e as usize][a as usize]) {
							canvas.fill_rect(Rect::new((a%3)*space_scale + place, a/3*space_scale, num_scale as u32, num_scale as u32));
						}
					}

				},
				_ => {},
			}
			place += (space_scale + num_scale) * 3;
		}

		// End Draw Frame Rate

		canvas.present();
	}
}
	
// Fully Functional
fn next_life(grid: &Vec<bool>, height:i32,width:i32) -> Vec<bool> {

	let rules = [
		[false, false, true, true, false, false, false, false, false], //Rules for living cells
		[false, false, false, true, false, false, false, false, false]]; //Rules for dead cells

	let mut new_grid: Vec<bool> = Vec::new();
	let x_off = [-1,0,1,1,1,0,-1,-1];
	let y_off = [1,1,1,0,-1,-1,-1,0];
	for y in 0..height {
		for x in 0..width {
			let pos = y * width + x;
			let mut neighbors = 0;
			for a in 0..8 {
				if 0 <= x_off[a]+x && x_off[a]+x < width && 0 <= y_off[a]+y && y_off[a]+y < height {
					if grid[((y_off[a]+y) * width + x_off[a]+x) as usize] {
						neighbors += 1;
					}
				}
			}
			if grid[pos as usize] {
				new_grid.push(rules[0][neighbors]);
			} else {
				new_grid.push(rules[1][neighbors]);
			}
		}
	}
	return new_grid;
}