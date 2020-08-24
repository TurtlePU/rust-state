pub struct Article;

impl Article {
    pub fn empty() -> Self {
        Self
    }

    pub fn add_text(&self, _text: &str) {
        // no-op
    }

    pub fn content(&self) -> Option<&str> {
        None
    }

    pub fn send_to_moderators(&self) {
        // no-op
    }

    pub fn publish(&self) {
        // no-op
    }
}
