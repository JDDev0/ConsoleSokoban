use std::collections::VecDeque;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct UndoHistory<T> {
    history: VecDeque<T>,
    next_index: usize,
}

impl<T> UndoHistory<T> {
    pub fn new(capacity: usize, initial: T) -> Self {
        if capacity == 0 {
            panic!("Capacity must be > 0");
        }

        let mut history = VecDeque::with_capacity(capacity);
        history.push_back(initial);

        Self {
            history,
            next_index: 1,
        }
    }

    pub fn undo(&mut self) -> Option<&T> {
        if self.next_index <= 1 {
            return None;
        }

        self.next_index -= 1;

        self.history.get(self.next_index - 1)
    }

    pub fn redo(&mut self) -> Option<&T> {
        if self.next_index == self.history.len() {
            return None;
        }

        let value = self.history.get(self.next_index);

        self.next_index += 1;

        value
    }

    pub fn commit_change(&mut self, value: T) {
        self.history.truncate(self.next_index);

        if self.history.len() == self.history.capacity() {
            self.history.pop_front();
        }else {
            self.next_index += 1;
        }

        self.history.push_back(value);
    }

    pub fn current(&mut self) -> &T {
        &self.history[self.next_index - 1]
    }

    pub fn clear(&mut self) {
        //Last element of history is current value and should be the new initial value
        self.history.swap_remove_back(0);
        self.history.truncate(1);
        self.next_index = 1;
    }

    pub fn clear_with_new_initial(&mut self, initial_value: T) {
        self.history.clear();
        self.history.push_back(initial_value);
        self.next_index = 1;
    }
}
