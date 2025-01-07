# 🎯 Typewriter

A modern terminal-based typewriter simulator written in Rust that provides a more deliberate writing experience. It features authentic typewriter sounds, delayed character reveal, and non-destructive editing.

## ✨ Features

- **🔊 Authentic Typewriter Sounds** - Different sounds for different character groups and a classic return sound
- **⌛ Delayed Character Reveal** - Characters appear with a slight delay, encouraging deliberate typing
- **✏️ Non-destructive Editing** - Backspace doesn't delete text but allows marking out characters
- **📜 Progressive Text Aging** - Older lines become dimmed, helping focus on current content

## 🚀 Quick Start

### Pre-compiled Binaries

Download the latest version for your platform from our [Releases page](https://github.com/ASBecker/typewriter/releases):

- **Windows**: `typewriter-windows-x86_64.zip`
- **macOS**: `typewriter-macos-x86_64.tar.gz` (Intel) or `typewriter-macos-aarch64.tar.gz` (Apple Silicon)
- **Linux**: `typewriter-linux-x86_64.tar.gz`

Each archive includes:
- The typewriter executable
- Sound files
- Documentation
- License information

### Building from Source

#### Prerequisites

- Rust and Cargo (install from [rustup.rs](https://rustup.rs))
- A terminal that supports ANSI escape codes
- Audio output device (optional, for sound effects)

#### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/typewriter.git
cd typewriter

# Build and run
cargo run
```

### Usage

```bash
# Start with a new document
typewriter

# Open an existing file
typewriter myfile.txt

# Enable typewriter sounds
typewriter --sound

# Open a file with sounds enabled
typewriter myfile.txt --sound
```

## ⌨️ Controls

- **Type normally** to enter text (with typewriter delay)
- **Backspace** to move back without deleting (enters mark-out mode)
- **'x'** in mark-out mode to cross out characters
- **Right Arrow** to move through text in mark-out mode
- **Enter** for new line (with classic carriage return sound)
- **Ctrl+S** to save
- **Ctrl+X** to exit (prompts to save if there are changes)

## 🎵 Sound System

The typewriter features an sound system that:
- Maps different character groups to distinct click sounds
- Adds subtle random variations in pitch (±5%) and volume (±10%)
- Plays a classic carriage return sound for line breaks
- Synchronizes sounds with visual character reveal

## 🛠️ Technical Details

Built with:
- **[Rust](https://www.rust-lang.org/)** - For performance and reliability
- **[crossterm](https://github.com/crossterm-rs/crossterm)** - Cross-platform terminal manipulation
- **[rodio](https://github.com/RustAudio/rodio)** - Audio playback
- **[tokio](https://tokio.rs/)** - Async runtime for non-blocking I/O

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/NewFeature`)
3. Commit your Changes (`git commit -m 'Add some NewFeature'`)
4. Push to the Branch (`git push origin feature/NewFeature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Original sound effects sourced from [pixabay.com](https://pixabay.com/sound-effects/search/typewriter/)
- Inspired by reddit user [u/AvalancheOfOpinions](https://www.reddit.com/r/ClaudeAI/comments/1hsxv7u/comment/m59lo6q/)