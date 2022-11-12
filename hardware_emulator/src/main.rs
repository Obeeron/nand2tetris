use pixels::SurfaceTexture;
use winit::{event_loop::EventLoop, dpi::LogicalSize, window::WindowBuilder};
use clap::{self, Parser};

mod emulator;
pub use emulator::*;

mod error;
pub use error::*;

mod keyboard;
pub use keyboard::*;

mod screen;
pub use screen::*;

mod cpu;
pub use cpu::*;

mod rom;
pub use rom::*;

mod memory;
pub use memory::*;

#[derive(Parser)]
#[command(author = "Obeeron", version = env!("CARGO_PKG_VERSION"), about="A Hack Computer emulator")]
struct Cli {
    hack_file: String,
    #[clap(short, long="cpy-cycle-per-sec", default_value = "2000000", help = "Number of CPU cycles per second")]
    cpu_cycles_per_sec: u32,
}

// Takes a path to a .hack file and executes it
fn main() -> Result<()>{
    let cli = Cli::parse();
    if cli.hack_file == "" {
        println!("No input file specified");
        return Ok(());
    }

    // Get ROM from binary file
    let rom = Rom::from_file(&cli.hack_file)?;

    // Create Event Loop
    let event_loop = EventLoop::new();
    
    // Create Window
    let screen_size = LogicalSize::new(512, 256);
    let scaled_screen_size = LogicalSize::new(512*2, 256*2);
    let window = WindowBuilder::new()
        .with_title("Hack Computer Emulator")
        .with_inner_size(scaled_screen_size)
        .with_min_inner_size(screen_size)
        .build(&event_loop)
        .unwrap();
    let surface_texture = SurfaceTexture::new(window.inner_size().width, window.inner_size().height, &window);
    
    // Create a new Pixel buffer
    let pixels = pixels::Pixels::new(512, 256, surface_texture).unwrap();

    // Create and run emulator
    let emulator = Emulator::new(rom, pixels);
    emulator.run(event_loop, window, cli.cpu_cycles_per_sec)
}
