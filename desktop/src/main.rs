use chip8_core::*;
use std::env;

use std::sync::Arc;

use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{self, Event, WindowEvent};
use winit::event_loop::{self, EventLoop};
use winit::keyboard::KeyCode;
use winit::window::Window;
use winit_input_helper::WinitInputHelper;

const SCALE: u32 = 15;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * SCALE;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("usage: cargo run path/to/game");
        return;
    }

    // Setup pixels
}
