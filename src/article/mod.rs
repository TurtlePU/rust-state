use state::State;
use states::Draft;

mod state;
mod states;

pub struct Article {
    state: Box<dyn State>,
    content: String,
}

impl Article {
    pub fn empty() -> Self {
        Self {
            state: Box::new(Draft),
            content: String::new(),
        }
    }

    pub fn add_text(&self, _text: &str) {
        // no-op
    }

    pub fn content(&self) -> Option<&str> {
        self.state.content(self)
    }

    pub fn send_to_moderators(&mut self) {
        self.state.send_to_moderators().apply(self);
    }

    pub fn publish(&mut self) {
        self.state.publish().apply(self);
    }
}
