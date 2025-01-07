mod editor;
mod input;
mod renderer;
mod sound;

use editor::Buffer;
use input::{InputEvent, InputHandler};
use renderer::Renderer;
use sound::{SoundSystem, SoundType};
use std::io::{self, stdout, Write};
use std::time::Duration;
use std::env;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Get filename from command line args if provided
    let args: Vec<String> = env::args().collect();
    let reveal_rate = Duration::from_millis(300);
    let input_timeout = Duration::from_millis(50);
    
    // Check if sound is enabled
    let sound_system = if args.contains(&"--sound".to_string()) {
        SoundSystem::new()
    } else {
        None
    };
    
    // Initialize buffer, either empty or from file
    let mut buffer = if args.len() > 1 && !args[1].starts_with("--") {
        Buffer::from_file(&args[1], reveal_rate)?
    } else {
        Buffer::new(reveal_rate)
    };

    let input_handler = InputHandler::new(input_timeout);
    let mut renderer = Renderer::new(stdout());

    // Set up terminal
    renderer.init()?;

    // Main event loop
    loop {
        // Render current state
        renderer.render(&buffer)?;

        // Handle input
        match input_handler.next_event().await? {
            InputEvent::Char(c) => {
                if let Some(sound_system) = &sound_system {
                    let reveal_time = std::time::Instant::now() + reveal_rate;
                    sound_system.schedule_sound(SoundType::KeyPress(c), reveal_time);
                }
                buffer.insert_char(c);
            }
            InputEvent::Backspace => buffer.backspace(),
            InputEvent::NewLine => {
                if let Some(sound_system) = &sound_system {
                    let reveal_time = std::time::Instant::now() + reveal_rate;
                    sound_system.schedule_sound(SoundType::Return, reveal_time);
                }
                buffer.new_line();
            }
            InputEvent::Right => buffer.move_right(),
            InputEvent::Save => {
                if buffer.file_path.is_none() {
                    // If no file path is set, prompt for one
                    renderer.cleanup()?;
                    print!("Enter filename to save: ");
                    io::stdout().flush()?;
                    let mut filename = String::new();
                    io::stdin().read_line(&mut filename)?;
                    buffer.set_file_path(filename.trim());
                    renderer.init()?;
                }
                
                if let Err(e) = buffer.save() {
                    renderer.cleanup()?;
                    eprintln!("Error saving file: {}", e);
                    std::thread::sleep(Duration::from_secs(2));
                    renderer.init()?;
                }
            }
            InputEvent::Close => {
                if buffer.is_modified() {
                    renderer.cleanup()?;
                    print!("Save changes before closing? (y/n) ");
                    io::stdout().flush()?;
                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;
                    if input.trim().to_lowercase().starts_with('y') {
                        if buffer.file_path.is_none() {
                            print!("Enter filename to save: ");
                            io::stdout().flush()?;
                            let mut filename = String::new();
                            io::stdin().read_line(&mut filename)?;
                            buffer.set_file_path(filename.trim());
                        }
                        buffer.save()?;
                    }
                }
                break;
            }
            InputEvent::Timeout => (), // Do nothing on timeout
        }
    }

    // Clean up
    renderer.cleanup()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    /// Test that the main components can be initialized
    async fn test_component_initialization() {
        let reveal_rate = Duration::from_millis(100);
        let input_timeout = Duration::from_millis(50);
        
        let buffer = Buffer::new(reveal_rate);
        let input_handler = InputHandler::new(input_timeout);
        let renderer = Renderer::new(Vec::new()); // Use Vec as a mock writer
        
        assert_eq!(buffer.reveal_rate(), reveal_rate);
        assert!(renderer.output.is_empty());
    }
}
