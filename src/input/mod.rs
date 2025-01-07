use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::time::Duration;

/// Represents different types of input events our editor can handle
#[derive(Debug, PartialEq)]
pub enum InputEvent {
    /// A regular character was typed
    Char(char),
    /// Backspace was pressed
    Backspace,
    /// Enter was pressed
    NewLine,
    /// Right arrow was pressed
    Right,
    /// Save command (Ctrl+S)
    Save,
    /// Close command (Ctrl+X)
    Close,
    /// No event occurred within timeout
    Timeout,
}

/// Handles keyboard input events
pub struct InputHandler {
    /// How long to wait for input before timing out
    timeout: Duration,
}

impl InputHandler {
    /// Creates a new input handler with the specified timeout
    pub fn new(timeout: Duration) -> Self {
        Self { timeout }
    }

    /// Reads the next input event, waiting up to timeout duration
    pub async fn next_event(&self) -> std::io::Result<InputEvent> {
        if event::poll(self.timeout)? {
            if let Event::Key(key) = event::read()? {
                Ok(self.handle_key_event(key))
            } else {
                Ok(InputEvent::Timeout)
            }
        } else {
            Ok(InputEvent::Timeout)
        }
    }

    /// Converts a key event into our InputEvent enum
    fn handle_key_event(&self, key: KeyEvent) -> InputEvent {
        match key.code {
            KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                InputEvent::Save
            }
            KeyCode::Char('x') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                InputEvent::Close
            }
            KeyCode::Char(c) => InputEvent::Char(c),
            KeyCode::Backspace => InputEvent::Backspace,
            KeyCode::Enter => InputEvent::NewLine,
            KeyCode::Right => InputEvent::Right,
            _ => InputEvent::Timeout,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    /// Test input handler creation
    async fn test_input_handler_creation() {
        let handler = InputHandler::new(Duration::from_millis(100));
        assert_eq!(handler.timeout, Duration::from_millis(100));
    }

    #[test]
    /// Test handling of various key events
    fn test_key_event_handling() {
        let handler = InputHandler::new(Duration::from_millis(100));

        // Test regular character
        let key = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::empty());
        assert_eq!(handler.handle_key_event(key), InputEvent::Char('a'));

        // Test backspace
        let key = KeyEvent::new(KeyCode::Backspace, KeyModifiers::empty());
        assert_eq!(handler.handle_key_event(key), InputEvent::Backspace);

        // Test enter
        let key = KeyEvent::new(KeyCode::Enter, KeyModifiers::empty());
        assert_eq!(handler.handle_key_event(key), InputEvent::NewLine);

        // Test right arrow
        let key = KeyEvent::new(KeyCode::Right, KeyModifiers::empty());
        assert_eq!(handler.handle_key_event(key), InputEvent::Right);

        // Test save (Ctrl+S)
        let key = KeyEvent::new(KeyCode::Char('s'), KeyModifiers::CONTROL);
        assert_eq!(handler.handle_key_event(key), InputEvent::Save);

        // Test close (Ctrl+X)
        let key = KeyEvent::new(KeyCode::Char('x'), KeyModifiers::CONTROL);
        assert_eq!(handler.handle_key_event(key), InputEvent::Close);
    }
} 