use super::state::{State, Transit};
use crate::article::Article;

pub struct Draft;

impl State for Draft {
    fn send_to_moderators(&mut self) -> Transit {
        Transit::to(PendingReview)
    }
}

pub struct PendingReview;

impl State for PendingReview {
    fn publish(&mut self) -> Transit {
        Transit::to(Published)
    }
}

pub struct Published;

impl State for Published {
    fn content<'a>(&self, article: &'a Article) -> Option<&'a str> {
        Some(&article.content)
    }
}
