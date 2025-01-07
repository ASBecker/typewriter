use crossterm::{
    cursor,
    style::{self, Stylize},
    terminal::{self, Clear, ClearType},
    QueueableCommand,
};
use std::io::{self, Write};
use std::time::Instant;

use crate::editor::{Buffer, Character, CharacterState};

/// Number of lines to keep fully visible
const VISIBLE_LINES: usize = 2;

/// Handles rendering the buffer to the terminal
pub struct Renderer<W: Write> {
    /// The output writer (usually stdout)
    output: W,
    /// Current cursor position in terminal coordinates
    cursor_pos: (u16, u16),
}

impl<W: Write> Renderer<W> {
    /// Creates a new renderer with the specified output
    pub fn new(output: W) -> Self {
        Self { 
            output,
            cursor_pos: (0, 0),
        }
    }

    /// Initializes the terminal for rendering
    pub fn init(&mut self) -> io::Result<()> {
        terminal::enable_raw_mode()?;
        self.output.queue(terminal::EnterAlternateScreen)?;
        self.output.queue(cursor::Show)?;
        self.output.flush()
    }

    /// Cleans up the terminal
    pub fn cleanup(&mut self) -> io::Result<()> {
        terminal::disable_raw_mode()?;
        self.output.queue(terminal::LeaveAlternateScreen)?;
        self.output.queue(cursor::Show)?;
        self.output.flush()
    }

    /// Renders the buffer to the terminal
    pub fn render(&mut self, buffer: &Buffer) -> io::Result<()> {
        // Clear the screen
        self.output.queue(Clear(ClearType::All))?;
        self.output.queue(cursor::MoveTo(0, 0))?;

        let now = Instant::now();
        let (cursor_line, cursor_col) = buffer.cursor_position();
        let is_mark_out_mode = buffer.is_mark_out_mode();

        // Calculate the number of lines that should be visible at full brightness
        let visible_start = buffer.lines.len().saturating_sub(VISIBLE_LINES);

        // Render each line
        for (line_idx, line) in buffer.lines.iter().enumerate() {
            // Move to the start of the current line
            self.output.queue(cursor::MoveTo(0, line_idx as u16))?;

            // Only dim lines that are above the visible region
            let should_dim = line_idx < visible_start;
            let is_current_line = line_idx == cursor_line;
            
            for (char_idx, character) in line.characters.iter().enumerate() {
                // Only show characters that have "matured" based on reveal rate
                if now.duration_since(character.timestamp) >= buffer.reveal_rate() {
                    // In mark-out mode, highlight characters from cursor position to end of line
                    let should_highlight = is_mark_out_mode && is_current_line && char_idx >= cursor_col;
                    self.render_character(character, should_dim, should_highlight)?;
                }
            }
            
            // Store cursor position if this is the current line
            if is_current_line {
                self.cursor_pos = (cursor_col as u16, line_idx as u16);
            }
            
            // Add newline after each line
            writeln!(self.output)?;
        }

        // Move cursor to its position
        self.output.queue(cursor::MoveTo(self.cursor_pos.0, self.cursor_pos.1))?;
        
        self.output.flush()
    }

    /// Renders a single character with appropriate styling
    fn render_character(&mut self, character: &Character, should_dim: bool, highlight: bool) -> io::Result<()> {
        let mut styled = match character.state {
            CharacterState::Normal => style::style(character.value),
            CharacterState::MarkedOut => style::style(character.value).crossed_out(),
        };

        // Apply dimming effect for older lines
        if should_dim {
            styled = styled.dim();
        }

        // Apply highlight effect if needed
        if highlight {
            styled = styled.reverse();
        }

        self.output.queue(style::PrintStyledContent(styled))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::Line;
    use std::time::Duration;

    /// A mock writer for testing
    struct MockWriter {
        contents: Vec<u8>,
    }

    impl MockWriter {
        fn new() -> Self {
            Self { contents: Vec::new() }
        }

        fn contents(&self) -> &[u8] {
            &self.contents
        }
    }

    impl Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.contents.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    #[test]
    /// Test renderer creation and basic operations
    fn test_renderer_creation() {
        let writer = MockWriter::new();
        let renderer = Renderer::new(writer);
        assert_eq!(renderer.cursor_pos, (0, 0));
    }

    #[test]
    /// Test character rendering
    fn test_character_rendering() {
        let mut writer = MockWriter::new();
        let mut renderer = Renderer::new(writer);
        
        let character = Character::new('a');
        renderer.render_character(&character, false, false).unwrap();
        
        // The output should contain the character 'a' plus some ANSI codes
        assert!(renderer.output.contents().contains(&b'a'));
    }

    #[test]
    /// Test marked out character rendering
    fn test_marked_out_rendering() {
        let mut writer = MockWriter::new();
        let mut renderer = Renderer::new(writer);
        
        let mut character = Character::new('a');
        character.mark_out();
        renderer.render_character(&character, false, false).unwrap();
        
        // The output should contain the character 'a' plus ANSI codes for strikethrough
        assert!(renderer.output.contents().contains(&b'a'));
        // Should contain ANSI codes for strikethrough (we don't test the exact codes as they might vary)
        assert!(renderer.output.contents().len() > 1);
    }
} 