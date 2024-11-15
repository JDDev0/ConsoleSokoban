use console_lib::{Color, Console};

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum DialogSelection {
    No,
    Yes,
}

#[allow(unused_variables)]
pub trait Dialog {
    fn draw(&self, console: &Console, console_width: usize, console_height: usize);

    fn on_key_pressed(&self, console_width: usize, console_height: usize, key: i32) -> Option<DialogSelection>;
    fn on_mouse_pressed(&self, console_width: usize, console_height: usize, column: usize, row: usize) -> Option<DialogSelection>;
}

pub struct DialogYesNo {
    message: String
}

impl DialogYesNo {
    pub fn new(message: impl Into<String>) -> Self {
        Self { message: message.into() }
    }
}

impl Dialog for DialogYesNo {
    fn draw(&self, console: &Console, console_width: usize, console_height: usize) {
        let char_count = self.message.chars().count();

        let width = char_count.max(16);
        let width_with_border = width + 2;

        let x_start = ((console_width - width_with_border) as f64 * 0.5) as usize;
        let y_start = ((console_height - 6) as f64 * 0.5) as usize;

        let whitespace_count_half = ((width - char_count) as f64 * 0.5) as usize;

        console.set_color(Color::Black, Color::Yellow);
        console.set_cursor_pos(x_start + 1, y_start + 1);
        console.draw_text(format!(
            "{}{}{}",
            " ".repeat(whitespace_count_half),
            self.message,
            " ".repeat(width - char_count - whitespace_count_half),
        ));

        console.set_cursor_pos(x_start + 1, y_start + 2);
        console.draw_text(format!(
            "{}{}{}",
            " ".repeat(whitespace_count_half),
            "-".repeat(char_count),
            " ".repeat(width - char_count - whitespace_count_half),
        ));

        console.set_cursor_pos(x_start + 1, y_start + 3);
        console.draw_text(" ".repeat(width));

        console.set_cursor_pos(x_start + 1, y_start + 4);
        console.draw_text(format!(
            "[y]es{}[n]o",
            " ".repeat(width - 9),
        ));

        //Draw border
        console.set_color(Color::LightBlack, Color::Red);
        console.set_cursor_pos(x_start, y_start);
        console.draw_text(" ".repeat(width_with_border));

        console.set_cursor_pos(x_start, y_start + 5);
        console.draw_text(" ".repeat(width_with_border));
        for i in y_start+1..y_start+5 {
            console.set_cursor_pos(x_start, i);
            console.draw_text(" ");

            console.set_cursor_pos(x_start + width_with_border - 1, i);
            console.draw_text(" ");
        }
    }

    fn on_key_pressed(&self, _: usize, _: usize, key: i32) -> Option<DialogSelection> {
        if key == b'y' as i32 {
            return Some(DialogSelection::Yes);
        }else if key == b'n' as i32 {
            return Some(DialogSelection::No);
        }

        None
    }

    fn on_mouse_pressed(&self, console_width: usize, console_height: usize, column: usize, row: usize) -> Option<DialogSelection> {
        let char_count = self.message.chars().count();

        let width = char_count.max(16);
        let width_with_border = width + 2;

        let x_start = ((console_width - width_with_border) as f64 * 0.5) as usize;
        let y_start = ((console_height - 6) as f64 * 0.5) as usize;

        if row == y_start + 4 {
            if (x_start + 1..x_start + 6).contains(&column) {
                return Some(DialogSelection::Yes);
            }else if (x_start + width - 3..x_start + width + 1).contains(&column) {
                return Some(DialogSelection::No);
            }
        }

        None
    }
}
