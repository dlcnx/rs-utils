pub struct LifoQueue<T> {
    content: Vec<T>,
}

impl<T> Default for LifoQueue<T> {
    fn default() -> Self {
        LifoQueue {
            content: Vec::new(),
        }
    }
}

impl<T> LifoQueue<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.content.len()
    }

    pub fn push(&mut self, val: T) {
        self.content.push(val);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.content.pop()
    }

    pub fn empty(&self) -> bool {
        if let 0 = self.len() {
            return true;
        }
        false
    }
}
