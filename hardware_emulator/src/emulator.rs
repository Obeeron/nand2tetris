use game_loop::{game_loop, GameLoop, Time};
use pixels::Pixels;
use winit::{event_loop::EventLoop, window::Window, event::{ Event,  DeviceEvent, ElementState, WindowEvent}};

use crate::{error::Result, keyboard::Keyboard, screen::Screen, Cpu, Rom, Memory, CpuOutput};

pub struct Emulator {
    pub rom: Rom,
    pub memory: Memory,
    pub cpu: Cpu,
    pub screen: Screen,
    pub keyboard: Keyboard,
} 

impl Emulator {
    pub fn new(rom: Rom, pixels: Pixels) -> Self {
        Emulator {
            rom,
            memory: Memory::new(),
            cpu: Cpu::new(),
            screen: Screen::new(pixels),
            keyboard: Keyboard::default(),
        }
    }

    pub fn run(mut self, event_loop: EventLoop<()>, window: Window, cpu_cycle_per_second: u32) -> Result<()> {
        println!("Running emulator");

        self.memory.store(0, 2000)?;
        game_loop(event_loop, window, self, cpu_cycle_per_second, 0.1, |g| {
            if let Err(e) = g.game.tick(){
                println!("{}", e);
                g.exit();
            }
        }, |g| {
            if let Err(e) = g.game.screen.render(){
                println!("{}", e);
                g.exit();
            }
        },  |g, event| {
            if let Err(e) = Self::handle_event(g, event) {
                println!("{}", e);
                g.exit();
            }
        });
    }

    fn handle_event(g: &mut GameLoop<Emulator, Time, Window>, event: &Event<()>) -> Result<()>{
        match event {
            Event::DeviceEvent {event, .. } => {
                if let DeviceEvent::Key(keyboard_input) = event {
                    if keyboard_input.state == ElementState::Released {
                        g.game.memory.set_keyboard_reg(0);
                    }
                    else {
                        let keycode= g.game.keyboard.keycode_from_winit(keyboard_input.scancode);
                        g.game.memory.set_keyboard_reg(keycode);
                    }
                }
            }
            Event::WindowEvent { window_id: _, event: WindowEvent::CloseRequested } => g.exit(),
            _ => {}
        }
        Ok(())
    }

    fn tick(&mut self) -> Result<()> {
        let instruction = self.rom.fetch(self.cpu.pc)?;
        
        let cpu_output: CpuOutput = self.cpu.execute(instruction, &self.memory)?;

        if cpu_output.write_to_ram {
            self.memory.store(cpu_output.memory_address, cpu_output.alu_out)?;
            if Memory::is_screen_address(cpu_output.memory_address) {
                self.screen.write(cpu_output.memory_address, cpu_output.alu_out)?;
            }
        }

        Ok(())
    }
}