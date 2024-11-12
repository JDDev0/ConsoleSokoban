use std::error::Error;
use std::ffi::c_int;
use std::fmt::{Display, Formatter};
use std::sync::{Mutex, MutexGuard};

mod bindings {
    use std::ffi::{c_char, c_int};

    extern "C" {
        pub fn clrscr();

        pub fn initConsole();
        pub fn reset();

        pub fn getConsoleSize(columns_ret: *mut c_int, rows_ret: *mut c_int);

        pub fn hasInput() -> c_int;
        pub fn getKey() -> c_int;

        pub fn getMousePosClicked(column: *mut c_int, row: *mut c_int);

        pub fn drawf(format: *const c_char, value: *const c_char);

        pub fn setColor(fg: c_int, bg: c_int);
        pub fn resetColor();

        pub fn setUnderline(underline: c_int);

        pub fn setCursorPos(x: c_int, y: c_int);
    }
}

pub mod keys {
    //Keys
    //Arrow keys
    pub const LEFT: i32 = 5000;
    pub const UP: i32 = 5001;
    pub const RIGHT: i32 = 5002;
    pub const DOWN: i32 = 5003;

    //F keys
    pub const F1: i32 = 5004;
    pub const F2: i32 = 5005;
    pub const F3: i32 = 5006;
    pub const F4: i32 = 5007;
    pub const F5: i32 = 5008;
    pub const F6: i32 = 5009;
    pub const F7: i32 = 5010;
    pub const F8: i32 = 5011;
    pub const F9: i32 = 5012;
    pub const F10: i32 = 5013;
    pub const F11: i32 = 5014;
    pub const F12: i32 = 5015;

    //Other keys
    pub const ESC: i32 = 5016;
    pub const DELETE: i32 = 5017;
    pub const ENTER: i32 = 5018;
}

pub fn is_arrow_key(key: i32) -> bool {
    (keys::LEFT..=keys::DOWN).contains(&key)
}

pub struct Console<'a> {
    _lock: MutexGuard<'a, ()>
}

static CONSOLE_MUTEX: Mutex<()> = Mutex::new(());

impl Console<'_> {
    pub fn new() -> Result<Self, Box<ConsoleError>> {
        let lock = match CONSOLE_MUTEX.try_lock() {
            Ok(lock) => lock,
            Err(_) => {
                return Err(Box::new(ConsoleError::new("Only one instance of Console can exist at once!")));
            },
        };

        unsafe { bindings::initConsole() };

        Ok(Self { _lock: lock })
    }

    pub fn repaint(&self) {
        unsafe { bindings::clrscr() }
    }

    /// Returns (x, y)
    pub fn get_console_size(&self) -> (usize, usize) {
        let mut columns_int: c_int = -1;
        let mut rows_int: c_int = -1;

        unsafe { bindings::getConsoleSize(&mut columns_int, &mut rows_int) }

        (columns_int as usize, rows_int as usize)
    }

    pub fn has_input(&self) -> bool {
        unsafe { bindings::hasInput() != 0 }
    }

    pub fn get_key(&self) -> i32 {
        unsafe { bindings::getKey() as i32 }
    }

    /// Returns (x, y)
    pub fn get_mouse_pos_clicked(&self) -> (isize, isize) {
        let mut column_int: c_int = -1;
        let mut row_int: c_int = -1;

        unsafe { bindings::getMousePosClicked(&mut column_int, &mut row_int) }

        (column_int as isize, row_int as isize)
    }

    pub fn draw_text(&self, text: impl Into<String>) {
        let format = std::ffi::CString::new("%s").unwrap();
        let text = std::ffi::CString::new(text.into()).unwrap();

        //TODO bounds checks (if cursor pos + text byte count > buffer size -> panic)

        unsafe { bindings::drawf(format.as_ptr(), text.as_ptr()) }
    }

    pub fn set_color(&self, fg: Color, bg: Color) {
        unsafe { bindings::setColor(fg as c_int, bg as c_int) }
    }

    pub fn reset_color(&self) {
        unsafe { bindings::resetColor() }
    }

    pub fn set_underline(&self, underline: bool) {
        unsafe { bindings::setUnderline(underline as c_int) }
    }

    pub fn set_cursor_pos(&self, x: usize, y: usize) {
        unsafe { bindings::setCursorPos(x as c_int, y as c_int) }
    }
}

impl Drop for Console<'_> {
    fn drop(&mut self) {
        unsafe { bindings::reset() };
    }
}

#[repr(i8)]
pub enum Color {
    Black,
    Blue,
    Green,
    Cyan,
    Red,
    Pink,
    Yellow,
    White,
    LightBlack,
    LightBlue,
    LightGreen,
    LightCyan,
    LightRed,
    LightPink,
    LightYellow,
    LightWhite,
    Default = -1
}

#[derive(Debug)]
pub struct ConsoleError {
    message: String
}

impl ConsoleError {
    fn new(message: impl Into<String>) -> Self {
        Self { message: message.into() }
    }
}

impl Display for ConsoleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl Error for ConsoleError {}
