pub struct Post {
    state: Option<dyn State>,
    content: String,
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
}


impl Post {
    fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push(text);
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {}

    pub fn content(&self) -> String {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
