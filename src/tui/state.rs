/// Application state.
#[derive(Debug)]
pub struct State {
    /// Is the application running?
    pub running: bool,
}

impl Default for State {
    fn default() -> Self {
        Self { running: true }
    }
}

impl State {
    /// Constructs a new instance of [`State`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Sets [`running`] to `false` to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}