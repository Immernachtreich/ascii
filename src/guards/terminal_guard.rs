use std::io::{ self, Result, Write };

use console::Term;

const ENTER_ALTERNATE_SCREEN: &str = "\x1b[?1049h";
const LEAVE_ALTERNATE_SCREEN: &str = "\x1b[?1049l";

pub struct TerminalGuard {
    pub term: Term,
}

impl TerminalGuard {
    pub fn new() -> Self {
        let term = Term::stdout();
        Self { term }
    }

    pub fn enter_alternate_screen(&self) -> Result<()> {
        let mut std_out = io::stdout();

        // Enter alternate screen
        std_out.write(ENTER_ALTERNATE_SCREEN.as_bytes())?;

        self.term.hide_cursor()?;
        self.term.clear_screen()?;
        self.term.move_cursor_to(0, 0)?;

        self.term.flush()?;
        Ok(())
    }

    pub fn leave_alternate_screen(&self) -> Result<()> {
        let mut std_out = io::stdout();

        // Leave alternate screen
        std_out.write(LEAVE_ALTERNATE_SCREEN.as_bytes()).unwrap();

        self.term.show_cursor().unwrap();
        self.term.flush().unwrap();

        Ok(())
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        self.leave_alternate_screen().expect("Failed to leave alternate screen");
    }
}
