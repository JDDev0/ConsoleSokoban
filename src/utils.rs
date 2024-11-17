use std::collections::VecDeque;

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
        self.history.truncate(1);
        self.next_index = 1;
    }

    pub fn clear_with_new_initial(&mut self, initial_value: T) {
        self.history.clear();
        self.history.push_back(initial_value);
        self.next_index = 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Capacity must be > 0")]
    fn invalid_capacity() {
        UndoHistory::<()>::new(0, ());
    }

    #[test]
    fn commit_change() {
        let mut undo_history = UndoHistory::new(5, 1);
        assert_eq!(undo_history.current(), &1);
        assert_eq!(undo_history.history.len(), 1);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 1);
        assert_eq!(undo_history.history[0], 1);

        undo_history.commit_change(2);
        assert_eq!(undo_history.current(), &2);
        assert_eq!(undo_history.history.len(), 2);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 2);
        assert_eq!(undo_history.history[0], 1);
        assert_eq!(undo_history.history[1], 2);

        undo_history.commit_change(3);
        assert_eq!(undo_history.current(), &3);
        assert_eq!(undo_history.history.len(), 3);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 3);
        assert_eq!(undo_history.history[0], 1);
        assert_eq!(undo_history.history[1], 2);
        assert_eq!(undo_history.history[2], 3);

        undo_history.commit_change(4);
        assert_eq!(undo_history.current(), &4);
        assert_eq!(undo_history.history.len(), 4);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 4);
        assert_eq!(undo_history.history[0], 1);
        assert_eq!(undo_history.history[1], 2);
        assert_eq!(undo_history.history[2], 3);
        assert_eq!(undo_history.history[3], 4);

        undo_history.commit_change(5);
        assert_eq!(undo_history.current(), &5);
        assert_eq!(undo_history.history.len(), 5);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 5);
        assert_eq!(undo_history.history[0], 1);
        assert_eq!(undo_history.history[1], 2);
        assert_eq!(undo_history.history[2], 3);
        assert_eq!(undo_history.history[3], 4);
        assert_eq!(undo_history.history[4], 5);

        undo_history.commit_change(6);
        assert_eq!(undo_history.current(), &6);
        assert_eq!(undo_history.history.len(), 5);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 5);
        assert_eq!(undo_history.history[0], 2);
        assert_eq!(undo_history.history[1], 3);
        assert_eq!(undo_history.history[2], 4);
        assert_eq!(undo_history.history[3], 5);
        assert_eq!(undo_history.history[4], 6);

        undo_history.commit_change(7);
        assert_eq!(undo_history.current(), &7);
        assert_eq!(undo_history.history.len(), 5);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 5);
        assert_eq!(undo_history.history[0], 3);
        assert_eq!(undo_history.history[1], 4);
        assert_eq!(undo_history.history[2], 5);
        assert_eq!(undo_history.history[3], 6);
        assert_eq!(undo_history.history[4], 7);
    }

    #[test]
    fn undo_changes_within_capacity() {
        let mut undo_history = UndoHistory::new(5, 1);

        assert_eq!(undo_history.undo(), None);
        assert_eq!(undo_history.current(), &1);
        assert_eq!(undo_history.history.len(), 1);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 1);

        assert_eq!(undo_history.undo(), None);
        assert_eq!(undo_history.current(), &1);
        assert_eq!(undo_history.history.len(), 1);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 1);

        assert_eq!(undo_history.undo(), None);
        assert_eq!(undo_history.current(), &1);
        assert_eq!(undo_history.history.len(), 1);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 1);

        undo_history.commit_change(2);
        undo_history.commit_change(3);

        assert_eq!(undo_history.undo(), Some(&2));
        assert_eq!(undo_history.current(), &2);
        assert_eq!(undo_history.history.len(), 3);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 2);

        assert_eq!(undo_history.undo(), Some(&1));
        assert_eq!(undo_history.current(), &1);
        assert_eq!(undo_history.history.len(), 3);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 1);

        undo_history.commit_change(4);
        undo_history.commit_change(5);
        undo_history.commit_change(6);
        undo_history.commit_change(7);
        undo_history.commit_change(8);

        assert_eq!(undo_history.undo(), Some(&7));
        assert_eq!(undo_history.current(), &7);
        assert_eq!(undo_history.history.len(), 5);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 4);

        assert_eq!(undo_history.undo(), Some(&6));
        assert_eq!(undo_history.current(), &6);
        assert_eq!(undo_history.history.len(), 5);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 3);

        assert_eq!(undo_history.undo(), Some(&5));
        assert_eq!(undo_history.current(), &5);
        assert_eq!(undo_history.history.len(), 5);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 2);

        assert_eq!(undo_history.undo(), Some(&4));
        assert_eq!(undo_history.current(), &4);
        assert_eq!(undo_history.history.len(), 5);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 1);

        assert_eq!(undo_history.undo(), None);
        assert_eq!(undo_history.current(), &4);
        assert_eq!(undo_history.history.len(), 5);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 1);

        assert_eq!(undo_history.undo(), None);
        assert_eq!(undo_history.current(), &4);
        assert_eq!(undo_history.history.len(), 5);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 1);
    }

    #[test]
    fn undo_changes_above_capacity() {
        let mut undo_history = UndoHistory::new(5, 1);
        undo_history.commit_change(2);
        undo_history.commit_change(3);
        undo_history.commit_change(4);
        undo_history.commit_change(5);
        undo_history.commit_change(6);

        assert_eq!(undo_history.undo(), Some(&5));
        assert_eq!(undo_history.current(), &5);
        assert_eq!(undo_history.history.len(), 5);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 4);

        assert_eq!(undo_history.undo(), Some(&4));
        assert_eq!(undo_history.current(), &4);
        assert_eq!(undo_history.history.len(), 5);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 3);

        assert_eq!(undo_history.undo(), Some(&3));
        assert_eq!(undo_history.current(), &3);
        assert_eq!(undo_history.history.len(), 5);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 2);

        assert_eq!(undo_history.undo(), Some(&2));
        assert_eq!(undo_history.current(), &2);
        assert_eq!(undo_history.history.len(), 5);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 1);

        assert_eq!(undo_history.undo(), None);
        assert_eq!(undo_history.current(), &2);
        assert_eq!(undo_history.history.len(), 5);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 1);

        assert_eq!(undo_history.undo(), None);
        assert_eq!(undo_history.current(), &2);
        assert_eq!(undo_history.history.len(), 5);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 1);

        assert_eq!(undo_history.undo(), None);
        assert_eq!(undo_history.current(), &2);
        assert_eq!(undo_history.history.len(), 5);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 1);
    }

    #[test]
    fn redo_changes_without_override() {
        let mut undo_history = UndoHistory::new(5, 1);
        undo_history.commit_change(2);
        undo_history.commit_change(3);
        undo_history.commit_change(4);
        undo_history.commit_change(5);

        assert_eq!(undo_history.redo(), None);
        assert_eq!(undo_history.current(), &5);
        assert_eq!(undo_history.history.len(), 5);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 5);

        undo_history.undo();

        assert_eq!(undo_history.redo(), Some(&5));
        assert_eq!(undo_history.current(), &5);
        assert_eq!(undo_history.history.len(), 5);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 5);

        assert_eq!(undo_history.redo(), None);
        assert_eq!(undo_history.current(), &5);
        assert_eq!(undo_history.history.len(), 5);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 5);

        undo_history.undo();
        undo_history.undo();
        undo_history.undo();
        undo_history.undo();
        undo_history.undo();

        assert_eq!(undo_history.redo(), Some(&2));
        assert_eq!(undo_history.current(), &2);
        assert_eq!(undo_history.history.len(), 5);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 2);

        assert_eq!(undo_history.redo(), Some(&3));
        assert_eq!(undo_history.current(), &3);
        assert_eq!(undo_history.history.len(), 5);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 3);

        assert_eq!(undo_history.redo(), Some(&4));
        assert_eq!(undo_history.current(), &4);
        assert_eq!(undo_history.history.len(), 5);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 4);

        assert_eq!(undo_history.redo(), Some(&5));
        assert_eq!(undo_history.current(), &5);
        assert_eq!(undo_history.history.len(), 5);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 5);

        assert_eq!(undo_history.redo(), None);
        assert_eq!(undo_history.current(), &5);
        assert_eq!(undo_history.history.len(), 5);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 5);

        assert_eq!(undo_history.redo(), None);
        assert_eq!(undo_history.current(), &5);
        assert_eq!(undo_history.history.len(), 5);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 5);

        undo_history.undo();
        undo_history.undo();
        undo_history.undo();
        undo_history.undo();
        undo_history.undo();
        undo_history.undo();
        undo_history.undo();
        undo_history.undo();
        undo_history.undo();

        assert_eq!(undo_history.redo(), Some(&2));
        assert_eq!(undo_history.current(), &2);
        assert_eq!(undo_history.history.len(), 5);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 2);

        assert_eq!(undo_history.redo(), Some(&3));
        assert_eq!(undo_history.current(), &3);
        assert_eq!(undo_history.history.len(), 5);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 3);

        assert_eq!(undo_history.redo(), Some(&4));
        assert_eq!(undo_history.current(), &4);
        assert_eq!(undo_history.history.len(), 5);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 4);

        assert_eq!(undo_history.redo(), Some(&5));
        assert_eq!(undo_history.current(), &5);
        assert_eq!(undo_history.history.len(), 5);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 5);

        assert_eq!(undo_history.redo(), None);
        assert_eq!(undo_history.current(), &5);
        assert_eq!(undo_history.history.len(), 5);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 5);

        assert_eq!(undo_history.redo(), None);
        assert_eq!(undo_history.current(), &5);
        assert_eq!(undo_history.history.len(), 5);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 5);
    }

    #[test]
    fn redo_changes_with_override() {
        let mut undo_history = UndoHistory::new(5, 1);
        undo_history.commit_change(2);
        undo_history.commit_change(3);
        undo_history.commit_change(4);
        undo_history.commit_change(5);

        undo_history.undo();
        undo_history.undo();
        undo_history.undo();
        undo_history.undo();

        assert_eq!(undo_history.redo(), Some(&2));
        assert_eq!(undo_history.current(), &2);
        assert_eq!(undo_history.history.len(), 5);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 2);

        undo_history.commit_change(42);
        assert_eq!(undo_history.current(), &42);
        assert_eq!(undo_history.history.len(), 3);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 3);

        assert_eq!(undo_history.redo(), None);
        assert_eq!(undo_history.current(), &42);
        assert_eq!(undo_history.history.len(), 3);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 3);

        undo_history.undo();

        undo_history.commit_change(84);
        assert_eq!(undo_history.current(), &84);
        assert_eq!(undo_history.history.len(), 3);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 3);

        assert_eq!(undo_history.redo(), None);
        assert_eq!(undo_history.history.len(), 3);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 3);

        undo_history.undo();
        undo_history.undo();
        undo_history.undo();

        undo_history.commit_change(21);
        assert_eq!(undo_history.current(), &21);
        assert_eq!(undo_history.history.len(), 2);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 2);

        assert_eq!(undo_history.undo(), Some(&1));
        assert_eq!(undo_history.current(), &1);
        assert_eq!(undo_history.history.len(), 2);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 1);

        assert_eq!(undo_history.redo(), Some(&21));
        assert_eq!(undo_history.current(), &21);
        assert_eq!(undo_history.history.len(), 2);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 2);

        assert_eq!(undo_history.redo(), None);
        assert_eq!(undo_history.current(), &21);
        assert_eq!(undo_history.history.len(), 2);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 2);
    }

    #[test]
    fn clear() {
        let mut undo_history = UndoHistory::new(5, 1);
        undo_history.commit_change(2);
        undo_history.commit_change(3);
        undo_history.commit_change(4);
        undo_history.commit_change(5);

        undo_history.undo();
        undo_history.undo();

        assert_eq!(undo_history.current(), &3);
        assert_eq!(undo_history.history.len(), 5);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 3);

        undo_history.clear();
        assert_eq!(undo_history.current(), &1);
        assert_eq!(undo_history.history.len(), 1);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 1);

        assert_eq!(undo_history.undo(), None);
        assert_eq!(undo_history.current(), &1);
        assert_eq!(undo_history.history.len(), 1);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 1);

        undo_history.commit_change(2);
        undo_history.commit_change(3);

        assert_eq!(undo_history.current(), &3);
        assert_eq!(undo_history.history.len(), 3);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 3);

        undo_history.clear();
        assert_eq!(undo_history.current(), &1);
        assert_eq!(undo_history.history.len(), 1);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 1);

        assert_eq!(undo_history.redo(), None);
        assert_eq!(undo_history.current(), &1);
        assert_eq!(undo_history.history.len(), 1);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 1);
    }

    #[test]
    fn clear_with_new_initial() {
        let mut undo_history = UndoHistory::new(5, 1);
        undo_history.commit_change(2);
        undo_history.commit_change(3);
        undo_history.commit_change(4);
        undo_history.commit_change(5);

        undo_history.undo();
        undo_history.undo();

        assert_eq!(undo_history.current(), &3);
        assert_eq!(undo_history.history.len(), 5);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 3);

        undo_history.clear_with_new_initial(42);
        assert_eq!(undo_history.current(), &42);
        assert_eq!(undo_history.history.len(), 1);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 1);

        assert_eq!(undo_history.undo(), None);
        assert_eq!(undo_history.current(), &42);
        assert_eq!(undo_history.history.len(), 1);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 1);

        undo_history.commit_change(2);
        undo_history.commit_change(3);

        assert_eq!(undo_history.current(), &3);
        assert_eq!(undo_history.history.len(), 3);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 3);

        undo_history.clear_with_new_initial(21);
        assert_eq!(undo_history.current(), &21);
        assert_eq!(undo_history.history.len(), 1);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 1);

        assert_eq!(undo_history.redo(), None);
        assert_eq!(undo_history.current(), &21);
        assert_eq!(undo_history.history.len(), 1);
        assert_eq!(undo_history.history.capacity(), 5);
        assert_eq!(undo_history.next_index, 1);
    }
}
