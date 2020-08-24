use crate::article::Article;

pub trait State {
    fn send_to_moderators(&mut self) -> Transit {
        Transit(None)
    }

    fn publish(&mut self) -> Transit {
        Transit(None)
    }
}

pub struct Transit(pub Option<Box<dyn State>>);

impl Transit {
    pub fn to(state: impl State + 'static) -> Self {
        Self(Some(Box::new(state)))
    }

    pub fn apply(self, article: &mut Article) -> Option<()> {
        article.state = self.0?;
        Some(())
    }
}
