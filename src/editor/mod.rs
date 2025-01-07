mod types;
pub use types::*;

use std::time::Duration;
use std::fs::{self, File};
use std::io;
use std::path::Path;

/// The main editor buffer that holds all text content
#[derive(Debug)]
pub struct Buffer {
    /// All lines in the buffer
    pub lines: Vec<Line>,
    /// Current line being edited
    current_line: usize,
    /// Current column position in the current line
    current_column: usize,
    /// Whether we're in mark-out mode (after backspace)
    mark_out_mode: bool,
    /// How many characters to reveal per second
    reveal_rate: Duration,
    /// The file path if the buffer is associated with a file
    pub file_path: Option<String>,
    /// Whether the buffer has unsaved changes
    is_modified: bool,
}

impl Buffer {
    /// Creates a new empty buffer with the specified reveal rate
    pub fn new(reveal_rate: Duration) -> Self {
        let mut lines = Vec::new();
        lines.push(Line::new());
        
        Self {
            lines,
            current_line: 0,
            current_column: 0,
            mark_out_mode: false,
            reveal_rate,
            file_path: None,
            is_modified: false,
        }
    }

    /// Creates a new buffer and loads content from the specified file.
    /// If the file doesn't exist, creates a new empty file.
    pub fn from_file(path: &str, reveal_rate: Duration) -> io::Result<Self> {
        let mut buffer = Self::new(reveal_rate);
        buffer.file_path = Some(path.to_string());

        // Create the file if it doesn't exist
        if !Path::new(path).exists() {
            File::create(path)?;
            buffer.lines.clear();
            buffer.lines.push(Line::new());
            return Ok(buffer);
        }

        // Load existing content
        let content = fs::read_to_string(path)?;
        
        // Split content into lines and populate buffer
        buffer.lines.clear();
        for line in content.lines() {
            let mut buffer_line = Line::new();
            for c in line.chars() {
                buffer_line.push(Character::new(c));
            }
            buffer.lines.push(buffer_line);
        }
        
        // Ensure there's at least one line
        if buffer.lines.is_empty() {
            buffer.lines.push(Line::new());
        }
        
        Ok(buffer)
    }

    /// Saves the buffer content to its associated file
    pub fn save(&mut self) -> io::Result<()> {
        if let Some(path) = &self.file_path {
            let mut content = String::new();
            
            // Convert buffer content to string
            for (i, line) in self.lines.iter().enumerate() {
                if i > 0 {
                    content.push('\n');
                }
                for character in &line.characters {
                    if character.state == CharacterState::Normal {
                        content.push(character.value);
                    }
                }
            }
            
            // Write to file
            fs::write(path, content)?;
            self.is_modified = false;
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "No file path associated with buffer"))
        }
    }

    /// Sets the file path for the buffer
    pub fn set_file_path(&mut self, path: &str) {
        self.file_path = Some(path.to_string());
    }

    /// Returns whether the buffer has unsaved changes
    pub fn is_modified(&self) -> bool {
        self.is_modified
    }

    /// Inserts a character at the current position
    pub fn insert_char(&mut self, c: char) {
        if self.mark_out_mode {
            // If we're in mark-out mode, mark out the character at current position
            let current_column = self.current_column;
            if let Some(character) = self.current_line_mut()
                .characters
                .get_mut(current_column) {
                character.mark_out();
                self.current_column += 1;
                // Only exit mark-out mode if we've reached the end of existing text
                if self.current_column >= self.current_line().characters.len() {
                    self.mark_out_mode = false;
                }
            }
        } else {
            // Normal insertion mode
            let character = Character::new(c);
            self.current_line_mut().push(character);
            self.current_column += 1;
        }
        self.is_modified = true;
    }

    /// Handles a backspace key press
    pub fn backspace(&mut self) {
        if self.current_column > 0 {
            self.current_column -= 1;
            self.mark_out_mode = true;
        } else if self.current_line > 0 {
            // Move to the end of the previous line
            self.current_line -= 1;
            self.current_column = self.current_line().characters.len();
            self.mark_out_mode = false;
        }
    }

    /// Handles a right arrow key press
    pub fn move_right(&mut self) {
        if self.current_column < self.current_line().characters.len() {
            self.current_column += 1;
            // Exit mark-out mode if we've reached the end of existing text
            if self.current_column >= self.current_line().characters.len() {
                self.mark_out_mode = false;
            }
        }
    }

    /// Handles an enter key press
    pub fn new_line(&mut self) {
        // Create a new line and move to it
        self.lines.push(Line::new());
        self.current_line += 1;
        self.current_column = 0;
        self.mark_out_mode = false;
    }

    /// Gets a reference to the current line
    fn current_line(&self) -> &Line {
        &self.lines[self.current_line]
    }

    /// Gets a mutable reference to the current line
    fn current_line_mut(&mut self) -> &mut Line {
        &mut self.lines[self.current_line]
    }

    /// Returns the reveal rate for characters
    pub fn reveal_rate(&self) -> Duration {
        self.reveal_rate
    }

    /// Returns the current cursor position (line, column)
    pub fn cursor_position(&self) -> (usize, usize) {
        (self.current_line, self.current_column)
    }

    /// Returns true if the cursor is in mark-out mode
    pub fn is_mark_out_mode(&self) -> bool {
        self.mark_out_mode
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Test basic buffer creation
    fn test_buffer_creation() {
        let buffer = Buffer::new(Duration::from_millis(100));
        assert_eq!(buffer.lines.len(), 1);
        assert_eq!(buffer.current_line, 0);
        assert_eq!(buffer.current_column, 0);
        assert!(!buffer.mark_out_mode);
    }

    #[test]
    /// Test character insertion
    fn test_character_insertion() {
        let mut buffer = Buffer::new(Duration::from_millis(100));
        
        buffer.insert_char('a');
        assert_eq!(buffer.current_line().characters[0].value, 'a');
        assert_eq!(buffer.current_column, 1);
        
        buffer.insert_char('b');
        assert_eq!(buffer.current_line().characters[1].value, 'b');
        assert_eq!(buffer.current_column, 2);
    }

    #[test]
    /// Test backspace and mark-out functionality
    fn test_backspace_and_mark_out() {
        let mut buffer = Buffer::new(Duration::from_millis(100));
        
        // Insert some characters
        buffer.insert_char('a');
        buffer.insert_char('b');
        buffer.insert_char('c');
        
        // Backspace and mark out
        buffer.backspace();
        assert!(buffer.mark_out_mode);
        assert_eq!(buffer.current_column, 2);
        
        // Mark out multiple characters
        buffer.insert_char('x');
        assert!(buffer.mark_out_mode); // Should still be in mark-out mode
        assert_eq!(buffer.current_line().characters[2].state, CharacterState::MarkedOut);
        
        buffer.insert_char('x');
        assert!(!buffer.mark_out_mode); // Should exit mark-out mode at end of text
    }

    #[test]
    /// Test right arrow movement
    fn test_move_right() {
        let mut buffer = Buffer::new(Duration::from_millis(100));
        
        // Insert some characters
        buffer.insert_char('a');
        buffer.insert_char('b');
        
        // Move back and then right
        buffer.backspace();
        assert!(buffer.mark_out_mode);
        
        buffer.move_right();
        assert!(buffer.mark_out_mode); // Should maintain mark-out mode
        assert_eq!(buffer.current_column, 2);
        
        // Should not move past end of text
        buffer.move_right();
        assert_eq!(buffer.current_column, 2);
        assert!(!buffer.mark_out_mode); // Should exit mark-out mode at end
    }

    #[test]
    /// Test new line functionality
    fn test_new_line() {
        let mut buffer = Buffer::new(Duration::from_millis(100));
        
        buffer.insert_char('a');
        buffer.new_line();
        
        assert_eq!(buffer.lines.len(), 2);
        assert_eq!(buffer.current_line, 1);
        assert_eq!(buffer.current_column, 0);
        assert!(!buffer.mark_out_mode);

        // Test that new line starts fresh
        buffer.insert_char('b');
        assert_eq!(buffer.current_line().characters[0].value, 'b');
        assert_eq!(buffer.current_column, 1);
    }

    #[test]
    /// Test backspace at start of line
    fn test_backspace_at_line_start() {
        let mut buffer = Buffer::new(Duration::from_millis(100));
        
        // Create two lines with text
        buffer.insert_char('a');
        buffer.new_line();
        buffer.insert_char('b');
        
        // Backspace at start of second line
        buffer.backspace();
        assert!(buffer.mark_out_mode);
        buffer.backspace();
        
        // Should move to end of previous line
        assert_eq!(buffer.current_line, 0);
        assert_eq!(buffer.current_column, 1);
        assert!(!buffer.mark_out_mode);
    }
} 