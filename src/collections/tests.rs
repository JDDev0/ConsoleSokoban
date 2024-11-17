use crate::collections::*;

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
    assert_eq!(undo_history.current_index, 0);
    assert_eq!(undo_history.history[0], 1);

    undo_history.commit_change(2);
    assert_eq!(undo_history.current(), &2);
    assert_eq!(undo_history.history.len(), 2);
    assert_eq!(undo_history.history.capacity(), 5);
    assert_eq!(undo_history.current_index, 1);
    assert_eq!(undo_history.history[0], 1);
    assert_eq!(undo_history.history[1], 2);

    undo_history.commit_change(3);
    assert_eq!(undo_history.current(), &3);
    assert_eq!(undo_history.history.len(), 3);
    assert_eq!(undo_history.history.capacity(), 5);
    assert_eq!(undo_history.current_index, 2);
    assert_eq!(undo_history.history[0], 1);
    assert_eq!(undo_history.history[1], 2);
    assert_eq!(undo_history.history[2], 3);

    undo_history.commit_change(4);
    assert_eq!(undo_history.current(), &4);
    assert_eq!(undo_history.history.len(), 4);
    assert_eq!(undo_history.history.capacity(), 5);
    assert_eq!(undo_history.current_index, 3);
    assert_eq!(undo_history.history[0], 1);
    assert_eq!(undo_history.history[1], 2);
    assert_eq!(undo_history.history[2], 3);
    assert_eq!(undo_history.history[3], 4);

    undo_history.commit_change(5);
    assert_eq!(undo_history.current(), &5);
    assert_eq!(undo_history.history.len(), 5);
    assert_eq!(undo_history.history.capacity(), 5);
    assert_eq!(undo_history.current_index, 4);
    assert_eq!(undo_history.history[0], 1);
    assert_eq!(undo_history.history[1], 2);
    assert_eq!(undo_history.history[2], 3);
    assert_eq!(undo_history.history[3], 4);
    assert_eq!(undo_history.history[4], 5);

    undo_history.commit_change(6);
    assert_eq!(undo_history.current(), &6);
    assert_eq!(undo_history.history.len(), 5);
    assert_eq!(undo_history.history.capacity(), 5);
    assert_eq!(undo_history.current_index, 4);
    assert_eq!(undo_history.history[0], 2);
    assert_eq!(undo_history.history[1], 3);
    assert_eq!(undo_history.history[2], 4);
    assert_eq!(undo_history.history[3], 5);
    assert_eq!(undo_history.history[4], 6);

    undo_history.commit_change(7);
    assert_eq!(undo_history.current(), &7);
    assert_eq!(undo_history.history.len(), 5);
    assert_eq!(undo_history.history.capacity(), 5);
    assert_eq!(undo_history.current_index, 4);
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
    assert_eq!(undo_history.current_index, 0);

    assert_eq!(undo_history.undo(), None);
    assert_eq!(undo_history.current(), &1);
    assert_eq!(undo_history.history.len(), 1);
    assert_eq!(undo_history.history.capacity(), 5);
    assert_eq!(undo_history.current_index, 0);

    assert_eq!(undo_history.undo(), None);
    assert_eq!(undo_history.current(), &1);
    assert_eq!(undo_history.history.len(), 1);
    assert_eq!(undo_history.history.capacity(), 5);
    assert_eq!(undo_history.current_index, 0);

    undo_history.commit_change(2);
    undo_history.commit_change(3);

    assert_eq!(undo_history.undo(), Some(&2));
    assert_eq!(undo_history.current(), &2);
    assert_eq!(undo_history.history.len(), 3);
    assert_eq!(undo_history.history.capacity(), 5);
    assert_eq!(undo_history.current_index, 1);

    assert_eq!(undo_history.undo(), Some(&1));
    assert_eq!(undo_history.current(), &1);
    assert_eq!(undo_history.history.len(), 3);
    assert_eq!(undo_history.history.capacity(), 5);
    assert_eq!(undo_history.current_index, 0);

    undo_history.commit_change(4);
    undo_history.commit_change(5);
    undo_history.commit_change(6);
    undo_history.commit_change(7);
    undo_history.commit_change(8);

    assert_eq!(undo_history.undo(), Some(&7));
    assert_eq!(undo_history.current(), &7);
    assert_eq!(undo_history.history.len(), 5);
    assert_eq!(undo_history.history.capacity(), 5);
    assert_eq!(undo_history.current_index, 3);

    assert_eq!(undo_history.undo(), Some(&6));
    assert_eq!(undo_history.current(), &6);
    assert_eq!(undo_history.history.len(), 5);
    assert_eq!(undo_history.history.capacity(), 5);
    assert_eq!(undo_history.current_index, 2);

    assert_eq!(undo_history.undo(), Some(&5));
    assert_eq!(undo_history.current(), &5);
    assert_eq!(undo_history.history.len(), 5);
    assert_eq!(undo_history.history.capacity(), 5);
    assert_eq!(undo_history.current_index, 1);

    assert_eq!(undo_history.undo(), Some(&4));
    assert_eq!(undo_history.current(), &4);
    assert_eq!(undo_history.history.len(), 5);
    assert_eq!(undo_history.history.capacity(), 5);
    assert_eq!(undo_history.current_index, 0);

    assert_eq!(undo_history.undo(), None);
    assert_eq!(undo_history.current(), &4);
    assert_eq!(undo_history.history.len(), 5);
    assert_eq!(undo_history.history.capacity(), 5);
    assert_eq!(undo_history.current_index, 0);

    assert_eq!(undo_history.undo(), None);
    assert_eq!(undo_history.current(), &4);
    assert_eq!(undo_history.history.len(), 5);
    assert_eq!(undo_history.history.capacity(), 5);
    assert_eq!(undo_history.current_index, 0);
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
    assert_eq!(undo_history.current_index, 3);

    assert_eq!(undo_history.undo(), Some(&4));
    assert_eq!(undo_history.current(), &4);
    assert_eq!(undo_history.history.len(), 5);
    assert_eq!(undo_history.history.capacity(), 5);
    assert_eq!(undo_history.current_index, 2);

    assert_eq!(undo_history.undo(), Some(&3));
    assert_eq!(undo_history.current(), &3);
    assert_eq!(undo_history.history.len(), 5);
    assert_eq!(undo_history.history.capacity(), 5);
    assert_eq!(undo_history.current_index, 1);

    assert_eq!(undo_history.undo(), Some(&2));
    assert_eq!(undo_history.current(), &2);
    assert_eq!(undo_history.history.len(), 5);
    assert_eq!(undo_history.history.capacity(), 5);
    assert_eq!(undo_history.current_index, 0);

    assert_eq!(undo_history.undo(), None);
    assert_eq!(undo_history.current(), &2);
    assert_eq!(undo_history.history.len(), 5);
    assert_eq!(undo_history.history.capacity(), 5);
    assert_eq!(undo_history.current_index, 0);

    assert_eq!(undo_history.undo(), None);
    assert_eq!(undo_history.current(), &2);
    assert_eq!(undo_history.history.len(), 5);
    assert_eq!(undo_history.history.capacity(), 5);
    assert_eq!(undo_history.current_index, 0);

    assert_eq!(undo_history.undo(), None);
    assert_eq!(undo_history.current(), &2);
    assert_eq!(undo_history.history.len(), 5);
    assert_eq!(undo_history.history.capacity(), 5);
    assert_eq!(undo_history.current_index, 0);
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
    assert_eq!(undo_history.current_index, 4);

    undo_history.undo();

    assert_eq!(undo_history.redo(), Some(&5));
    assert_eq!(undo_history.current(), &5);
    assert_eq!(undo_history.history.len(), 5);
    assert_eq!(undo_history.history.capacity(), 5);
    assert_eq!(undo_history.current_index, 4);

    assert_eq!(undo_history.redo(), None);
    assert_eq!(undo_history.current(), &5);
    assert_eq!(undo_history.history.len(), 5);
    assert_eq!(undo_history.history.capacity(), 5);
    assert_eq!(undo_history.current_index, 4);

    undo_history.undo();
    undo_history.undo();
    undo_history.undo();
    undo_history.undo();
    undo_history.undo();

    assert_eq!(undo_history.redo(), Some(&2));
    assert_eq!(undo_history.current(), &2);
    assert_eq!(undo_history.history.len(), 5);
    assert_eq!(undo_history.history.capacity(), 5);
    assert_eq!(undo_history.current_index, 1);

    assert_eq!(undo_history.redo(), Some(&3));
    assert_eq!(undo_history.current(), &3);
    assert_eq!(undo_history.history.len(), 5);
    assert_eq!(undo_history.history.capacity(), 5);
    assert_eq!(undo_history.current_index, 2);

    assert_eq!(undo_history.redo(), Some(&4));
    assert_eq!(undo_history.current(), &4);
    assert_eq!(undo_history.history.len(), 5);
    assert_eq!(undo_history.history.capacity(), 5);
    assert_eq!(undo_history.current_index, 3);

    assert_eq!(undo_history.redo(), Some(&5));
    assert_eq!(undo_history.current(), &5);
    assert_eq!(undo_history.history.len(), 5);
    assert_eq!(undo_history.history.capacity(), 5);
    assert_eq!(undo_history.current_index, 4);

    assert_eq!(undo_history.redo(), None);
    assert_eq!(undo_history.current(), &5);
    assert_eq!(undo_history.history.len(), 5);
    assert_eq!(undo_history.history.capacity(), 5);
    assert_eq!(undo_history.current_index, 4);

    assert_eq!(undo_history.redo(), None);
    assert_eq!(undo_history.current(), &5);
    assert_eq!(undo_history.history.len(), 5);
    assert_eq!(undo_history.history.capacity(), 5);
    assert_eq!(undo_history.current_index, 4);

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
    assert_eq!(undo_history.current_index, 1);

    assert_eq!(undo_history.redo(), Some(&3));
    assert_eq!(undo_history.current(), &3);
    assert_eq!(undo_history.history.len(), 5);
    assert_eq!(undo_history.history.capacity(), 5);
    assert_eq!(undo_history.current_index, 2);

    assert_eq!(undo_history.redo(), Some(&4));
    assert_eq!(undo_history.current(), &4);
    assert_eq!(undo_history.history.len(), 5);
    assert_eq!(undo_history.history.capacity(), 5);
    assert_eq!(undo_history.current_index, 3);

    assert_eq!(undo_history.redo(), Some(&5));
    assert_eq!(undo_history.current(), &5);
    assert_eq!(undo_history.history.len(), 5);
    assert_eq!(undo_history.history.capacity(), 5);
    assert_eq!(undo_history.current_index, 4);

    assert_eq!(undo_history.redo(), None);
    assert_eq!(undo_history.current(), &5);
    assert_eq!(undo_history.history.len(), 5);
    assert_eq!(undo_history.history.capacity(), 5);
    assert_eq!(undo_history.current_index, 4);

    assert_eq!(undo_history.redo(), None);
    assert_eq!(undo_history.current(), &5);
    assert_eq!(undo_history.history.len(), 5);
    assert_eq!(undo_history.history.capacity(), 5);
    assert_eq!(undo_history.current_index, 4);
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
    assert_eq!(undo_history.current_index, 1);

    undo_history.commit_change(42);
    assert_eq!(undo_history.current(), &42);
    assert_eq!(undo_history.history.len(), 3);
    assert_eq!(undo_history.history.capacity(), 5);
    assert_eq!(undo_history.current_index, 2);

    assert_eq!(undo_history.redo(), None);
    assert_eq!(undo_history.current(), &42);
    assert_eq!(undo_history.history.len(), 3);
    assert_eq!(undo_history.history.capacity(), 5);
    assert_eq!(undo_history.current_index, 2);

    undo_history.undo();

    undo_history.commit_change(84);
    assert_eq!(undo_history.current(), &84);
    assert_eq!(undo_history.history.len(), 3);
    assert_eq!(undo_history.history.capacity(), 5);
    assert_eq!(undo_history.current_index, 2);

    assert_eq!(undo_history.redo(), None);
    assert_eq!(undo_history.history.len(), 3);
    assert_eq!(undo_history.history.capacity(), 5);
    assert_eq!(undo_history.current_index, 2);

    undo_history.undo();
    undo_history.undo();
    undo_history.undo();

    undo_history.commit_change(21);
    assert_eq!(undo_history.current(), &21);
    assert_eq!(undo_history.history.len(), 2);
    assert_eq!(undo_history.history.capacity(), 5);
    assert_eq!(undo_history.current_index, 1);

    assert_eq!(undo_history.undo(), Some(&1));
    assert_eq!(undo_history.current(), &1);
    assert_eq!(undo_history.history.len(), 2);
    assert_eq!(undo_history.history.capacity(), 5);
    assert_eq!(undo_history.current_index, 0);

    assert_eq!(undo_history.redo(), Some(&21));
    assert_eq!(undo_history.current(), &21);
    assert_eq!(undo_history.history.len(), 2);
    assert_eq!(undo_history.history.capacity(), 5);
    assert_eq!(undo_history.current_index, 1);

    assert_eq!(undo_history.redo(), None);
    assert_eq!(undo_history.current(), &21);
    assert_eq!(undo_history.history.len(), 2);
    assert_eq!(undo_history.history.capacity(), 5);
    assert_eq!(undo_history.current_index, 1);
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
    assert_eq!(undo_history.current_index, 2);

    undo_history.clear();
    assert_eq!(undo_history.current(), &5);
    assert_eq!(undo_history.history.len(), 1);
    assert_eq!(undo_history.history.capacity(), 5);
    assert_eq!(undo_history.current_index, 0);

    assert_eq!(undo_history.undo(), None);
    assert_eq!(undo_history.current(), &5);
    assert_eq!(undo_history.history.len(), 1);
    assert_eq!(undo_history.history.capacity(), 5);
    assert_eq!(undo_history.current_index, 0);

    undo_history.commit_change(2);
    undo_history.commit_change(3);

    assert_eq!(undo_history.current(), &3);
    assert_eq!(undo_history.history.len(), 3);
    assert_eq!(undo_history.history.capacity(), 5);
    assert_eq!(undo_history.current_index, 2);

    undo_history.clear();
    assert_eq!(undo_history.current(), &3);
    assert_eq!(undo_history.history.len(), 1);
    assert_eq!(undo_history.history.capacity(), 5);
    assert_eq!(undo_history.current_index, 0);

    assert_eq!(undo_history.redo(), None);
    assert_eq!(undo_history.current(), &3);
    assert_eq!(undo_history.history.len(), 1);
    assert_eq!(undo_history.history.capacity(), 5);
    assert_eq!(undo_history.current_index, 0);
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
    assert_eq!(undo_history.current_index, 2);

    undo_history.clear_with_new_initial(42);
    assert_eq!(undo_history.current(), &42);
    assert_eq!(undo_history.history.len(), 1);
    assert_eq!(undo_history.history.capacity(), 5);
    assert_eq!(undo_history.current_index, 0);

    assert_eq!(undo_history.undo(), None);
    assert_eq!(undo_history.current(), &42);
    assert_eq!(undo_history.history.len(), 1);
    assert_eq!(undo_history.history.capacity(), 5);
    assert_eq!(undo_history.current_index, 0);

    undo_history.commit_change(2);
    undo_history.commit_change(3);

    assert_eq!(undo_history.current(), &3);
    assert_eq!(undo_history.history.len(), 3);
    assert_eq!(undo_history.history.capacity(), 5);
    assert_eq!(undo_history.current_index, 2);

    undo_history.clear_with_new_initial(21);
    assert_eq!(undo_history.current(), &21);
    assert_eq!(undo_history.history.len(), 1);
    assert_eq!(undo_history.history.capacity(), 5);
    assert_eq!(undo_history.current_index, 0);

    assert_eq!(undo_history.redo(), None);
    assert_eq!(undo_history.current(), &21);
    assert_eq!(undo_history.history.len(), 1);
    assert_eq!(undo_history.history.capacity(), 5);
    assert_eq!(undo_history.current_index, 0);
}
