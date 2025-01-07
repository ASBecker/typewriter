use std::time::Instant;

/// Represents the state of a character in the editor
#[derive(Debug, Clone, PartialEq)]
pub enum CharacterState {
    /// A normal, visible character
    Normal,
    /// A character that has been marked out (crossed out)
    MarkedOut,
}

/// Represents a single character in the editor buffer
#[derive(Debug, Clone)]
pub struct Character {
    /// The actual character
    pub value: char,
    /// The current state of the character
    pub state: CharacterState,
    /// When this character was typed
    pub timestamp: Instant,
}

impl Character {
    /// Creates a new character in normal state
    pub fn new(value: char) -> Self {
        Self {
            value,
            state: CharacterState::Normal,
            timestamp: Instant::now(),
        }
    }

    /// Marks out this character (crosses it out)
    pub fn mark_out(&mut self) {
        self.state = CharacterState::MarkedOut;
    }
}

/// Represents a line of text in the editor
#[derive(Debug, Clone)]
pub struct Line {
    /// The characters in this line
    pub characters: Vec<Character>,
}

impl Line {
    /// Creates a new empty line
    pub fn new() -> Self {
        Self {
            characters: Vec::new(),
        }
    }

    /// Adds a character to this line
    pub fn push(&mut self, character: Character) {
        self.characters.push(character);
    }

    /// Returns the number of characters in this line
    pub fn len(&self) -> usize {
        self.characters.len()
    }

    /// Returns true if this line has no characters
    pub fn is_empty(&self) -> bool {
        self.characters.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    /// Test creating a new character
    fn test_new_character() {
        let c = Character::new('a');
        assert_eq!(c.value, 'a');
        assert_eq!(c.state, CharacterState::Normal);
    }

    #[test]
    /// Test marking out a character
    fn test_mark_out_character() {
        let mut c = Character::new('a');
        c.mark_out();
        assert_eq!(c.state, CharacterState::MarkedOut);
    }

    #[test]
    /// Test character timestamp is set correctly
    fn test_character_timestamp() {
        let before = Instant::now();
        thread::sleep(Duration::from_millis(10));
        let c = Character::new('a');
        thread::sleep(Duration::from_millis(10));
        let after = Instant::now();
        
        assert!(c.timestamp >= before && c.timestamp <= after);
    }

    #[test]
    /// Test creating and manipulating a line
    fn test_line_operations() {
        let mut line = Line::new();
        assert!(line.is_empty());
        assert_eq!(line.len(), 0);

        line.push(Character::new('a'));
        assert!(!line.is_empty());
        assert_eq!(line.len(), 1);
        assert_eq!(line.characters[0].value, 'a');
    }
} 