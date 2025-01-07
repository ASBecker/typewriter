use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::{Duration, Instant};
use rand::Rng;
use std::thread;
use std::path::PathBuf;

/// Different types of sounds that can be played
#[derive(Debug, Clone)]
pub enum SoundType {
    /// Regular keypress sound (click1-6.wav)
    KeyPress(char),
    /// Enter key sound (classic-return.wav)
    Return,
}

/// A sound request with timing information
#[derive(Debug)]
struct SoundRequest {
    sound_type: SoundType,
    play_at: Instant,
}

/// Manages sound playback for the typewriter
pub struct SoundSystem {
    sender: Sender<SoundRequest>,
    #[allow(dead_code)]
    stream: OutputStream, // Keep the stream alive
}

impl SoundSystem {
    /// Creates a new sound system and starts the audio thread
    pub fn new() -> Option<Self> {
        // Try to initialize audio output
        match OutputStream::try_default() {
            Ok((stream, stream_handle)) => {
                let (sender, receiver) = mpsc::channel();

                // Verify sound files exist
                let sound_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("sounds");
                if !sound_dir.exists() {
                    eprintln!("Sound directory not found at: {}", sound_dir.display());
                    return None;
                }

                // Check if at least one sound file exists
                let test_file = sound_dir.join("click1.wav");
                if !test_file.exists() {
                    eprintln!("Sound files not found in: {}", sound_dir.display());
                    return None;
                }

                // Start audio thread
                let sound_dir_clone = sound_dir.clone();
                thread::spawn(move || {
                    Self::audio_thread(receiver, stream_handle, sound_dir_clone);
                });

                Some(Self { sender, stream })
            }
            Err(e) => {
                eprintln!("Failed to initialize audio: {}", e);
                None
            }
        }
    }

    /// Schedules a sound to be played
    pub fn schedule_sound(&self, sound_type: SoundType, reveal_time: Instant) {
        // Schedule sound to play 100ms before reveal
        let play_at = reveal_time - Duration::from_millis(100);
        let request = SoundRequest { sound_type, play_at };
        if let Err(e) = self.sender.send(request) {
            eprintln!("Failed to schedule sound: {}", e);
        }
    }

    /// Loads and decodes a sound file
    fn load_sound(path: PathBuf) -> Option<Decoder<BufReader<File>>> {
        match File::open(&path) {
            Ok(file) => {
                let reader = BufReader::new(file);
                match Decoder::new(reader) {
                    Ok(decoder) => Some(decoder),
                    Err(e) => {
                        eprintln!("Failed to decode sound file {}: {}", path.display(), e);
                        None
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to open sound file {}: {}", path.display(), e);
                None
            }
        }
    }

    /// Audio processing thread
    fn audio_thread(
        receiver: Receiver<SoundRequest>,
        stream_handle: rodio::OutputStreamHandle,
        sound_dir: PathBuf,
    ) {
        let mut rng = rand::thread_rng();

        while let Ok(request) = receiver.recv() {
            // Wait until it's time to play the sound
            let now = Instant::now();
            if request.play_at > now {
                thread::sleep(request.play_at - now);
            }

            // Create a new sink for this sound
            match Sink::try_new(&stream_handle) {
                Ok(sink) => {
                    match request.sound_type {
                        SoundType::KeyPress(c) => {
                            // Select sound based on character
                            let sound_idx = match c {
                                'a'..='f' => 1,
                                'g'..='l' => 2,
                                'm'..='r' => 3,
                                's'..='x' => 4,
                                'y'..='z' => 5,
                                _ => 6,
                            };

                            // Load and play the sound
                            let sound_path = sound_dir.join(format!("click{}.wav", sound_idx));
                            if let Some(sound) = Self::load_sound(sound_path) {
                                // Apply random pitch/volume
                                let speed = 0.95 + rng.gen::<f32>() * 0.1; // Random pitch ±5%
                                let volume = 0.9 + rng.gen::<f32>() * 0.2; // Random volume ±10%
                                sink.set_speed(speed);
                                sink.set_volume(volume);
                                sink.append(sound);
                                sink.detach();
                            }
                        }
                        SoundType::Return => {
                            // Load and play return sound at 20% volume
                            let return_path = sound_dir.join("classic-return.wav");
                            if let Some(sound) = Self::load_sound(return_path) {
                                sink.set_volume(0.2);
                                sink.append(sound);
                                sink.detach();
                            }
                        }
                    }
                }
                Err(e) => eprintln!("Failed to create audio sink: {}", e),
            }
        }
    }
}

impl Drop for SoundSystem {
    fn drop(&mut self) {
        // Channel will be closed when SoundSystem is dropped
    }
} 