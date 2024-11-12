use console_lib::{keys, Console};
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::error::Error;
use std::ffi::OsString;
use std::fmt::{Debug, Display, Formatter};
use crate::game::help_page::HelpPage;
use crate::game::level::LevelPack;
use crate::game::screen::{Dialog, Screen, ScreenId, ScreenInGame, ScreenSelectLevel, ScreenSelectLevelPack, ScreenStartMenu};

mod level;
mod screen;
mod help_page;

pub struct Game<'a> {
    console: &'a Console<'a>,

    screens: HashMap<ScreenId, Box<dyn Screen>>,
    current_screen_id: RefCell<ScreenId>,

    help_page: HelpPage,
    is_help: RefCell<bool>,
    dialog: RefCell<Option<Dialog>>,

    current_level_pack_index: RefCell<usize>,
    level_packs: RefCell<Vec<LevelPack>>,

    current_level_index: RefCell<usize>,

    is_player_background: RefCell<bool>,
    player_background_tmp: RefCell<i32>,

    should_exit: RefCell<bool>,
}

impl <'a> Game<'a> {
    pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    pub const CONSOLE_MIN_WIDTH: usize = 74;
    pub const CONSOLE_MIN_HEIGHT: usize = 23;

    const PLAYER_BACKGROUND_DELAY: i32 = 12;

    const SAVE_GAME_FOLDER: &'static str = "ConsoleSokoban";

    const MAP_TUTORIAL: &'static str = include_str!("../resources/tutorial.lvl");
    const MAP_MAIN: &'static str = include_str!("../resources/main.lvl");
    const MAP_SPECIAL: &'static str = include_str!("../resources/special.lvl");
    const MAP_DEMON: &'static str = include_str!("../resources/demon.lvl");

    pub fn get_or_create_save_game_folder() -> Result<OsString, Box<dyn Error>> {
        let mut directory = if cfg!(windows) {
            let mut home_drive = std::env::var_os("HOMEDRIVE").
                    ok_or(GameError::new("$HOMEDRIVE not set!"))?;
            home_drive.push(&std::env::var_os("HOMEPATH").
                    ok_or(GameError::new("$HOMEPATH not set!"))?);

            home_drive
        }else {
            std::env::var_os("HOME").
                    ok_or(GameError::new("$HOME not set!"))?
        };

        directory.push("/.jddev0/");
        directory.push(Self::SAVE_GAME_FOLDER);
        std::fs::create_dir_all(&directory)?;

        directory.push("/");
        Ok(directory)
    }

    pub fn new(console: &'a Console) -> Result<Self, Box<dyn Error>> {
        let (width, height) = console.get_console_size();
        if width < Self::CONSOLE_MIN_WIDTH || height < Self::CONSOLE_MIN_HEIGHT {
            return Err(Box::new(GameError::new(
                format_args!(
                    "Console is to small (Min: {} x {})!",
                    Self::CONSOLE_MIN_WIDTH,
                    Self::CONSOLE_MIN_HEIGHT
                ).to_string())
            ));
        }

        let screens = HashMap::from_iter([
            (ScreenId::StartMenu, Box::new(ScreenStartMenu::new()) as Box<dyn Screen>),
            (ScreenId::SelectLevelPack, Box::new(ScreenSelectLevelPack::new()) as Box<dyn Screen>),
            (ScreenId::SelectLevel, Box::new(ScreenSelectLevel::new()) as Box<dyn Screen>),
            (ScreenId::InGame, Box::new(ScreenInGame::new()) as Box<dyn Screen>),
        ]);

        let mut level_packs = Vec::with_capacity(LevelPack::MAX_LEVEL_PACK_COUNT);
        level_packs.append(&mut vec![
            LevelPack::read_from_save_game("tutorial", "build-in:tutorial", Self::MAP_TUTORIAL)?,
            LevelPack::read_from_save_game("main", "build-in:main", Self::MAP_MAIN)?,
            LevelPack::read_from_save_game("special", "build-in:special", Self::MAP_SPECIAL)?,
            LevelPack::read_from_save_game("demon", "build-in:demon", Self::MAP_DEMON)?,
        ]);

        //TODO load level packs from command line arguments

        //TODO check if any level is too large
        /*
                //"height >=", 1st line: infos
                if(width > gameMinWidth || height >= gameMinHeight) {
                    reset();
                    printf("Level is too large (Max: %d x %d) (Level: %d x %d)!\n", gameMinWidth,
                    gameMinHeight - 1, width, height);

                    exit(EXIT_FAILURE);
                }
         */

        Ok(Self {
            console,

            screens,
            current_screen_id: RefCell::new(ScreenId::StartMenu),

            help_page: HelpPage::new(),
            is_help: Default::default(),
            dialog: Default::default(),

            current_level_pack_index: Default::default(),
            level_packs: RefCell::new(level_packs),

            current_level_index: Default::default(),

            is_player_background: Default::default(),
            player_background_tmp: Default::default(),

            should_exit: Default::default(),
        })
    }

    #[must_use]
    pub fn update(&self) -> bool {
        if *self.should_exit.borrow() {
            return true;
        }

        if self.console.has_input() {
            self.update_key(self.console.get_key());
        }

        self.update_mouse();

        if !*self.is_help.borrow() {
            let screen = self.screens.get(&self.current_screen_id.borrow());
            if let Some(screen) = screen {
                screen.update(self);
            }
        }

        //Player background
        self.player_background_tmp.replace_with(|current| *current + 1);
        if *self.player_background_tmp.borrow() >= Self::PLAYER_BACKGROUND_DELAY + *self.is_player_background.borrow() as i32 {
            //If isPlayerBackground: wait an additional update (25 updates per second, every half
            //second: switch background/foreground colors [12 updates, 13 updates])
            self.player_background_tmp.replace(0);
            self.is_player_background.replace_with(|current| !*current);
        }

        self.draw();

        false
    }

    fn update_key(&self, key: i32) {
        let screen = self.screens.get(&self.current_screen_id.borrow());
        if *self.is_help.borrow() {
            if key == keys::F1 || key == keys::ESC {
                self.close_help_page();

                if let Some(screen) = screen {
                    screen.on_continue(self);
                }
            }else {
                self.help_page.on_key_pressed(key);
            }

            return;
        }

        if let Some(screen) = screen {
            screen.on_key_pressed(self, key);
        }
    }

    fn update_mouse(&self) {
        let (column, row) = self.console.get_mouse_pos_clicked();
        if column < 0 || row < 0 {
            return;
        }

        let (column, row) = (column as usize, row as usize);

        if *self.is_help.borrow() {
            self.help_page.on_mouse_pressed(Self::CONSOLE_MIN_WIDTH, Self::CONSOLE_MIN_HEIGHT, column, row);

            return;
        }

        let yes_no;
        if let Some(dialog) = self.dialog.borrow().as_ref() {
            yes_no = dialog.on_mouse_pressed(Self::CONSOLE_MIN_WIDTH, Self::CONSOLE_MIN_HEIGHT, column, row);
        }else {
            yes_no = None;
        }
        if let Some(yes_no) = yes_no {
            self.update_key(if yes_no {
                b'y' as i32
            }else {
                b'n' as i32
            });

            return;
        }

        let screen = self.screens.get(&self.current_screen_id.borrow());
        if let Some(screen) = screen {
            screen.on_mouse_pressed(self, column, row);
        }
    }

    fn draw(&self) {
        self.console.repaint();

        if *self.is_help.borrow() {
            self.help_page.draw(self.console, Self::CONSOLE_MIN_WIDTH, Self::CONSOLE_MIN_HEIGHT);

            return;
        }

        let screen = self.screens.get(&self.current_screen_id.borrow());
        if let Some(screen) = screen {
            screen.draw(self, self.console);
        }

        if let Some(dialog) = self.dialog.borrow().as_ref() {
            dialog.draw(self.console, Self::CONSOLE_MIN_WIDTH, Self::CONSOLE_MIN_HEIGHT);
        }
    }

    pub fn set_screen(&self, screen_id: ScreenId) {
        self.current_screen_id.replace(screen_id);

        let screen = self.screens.get(&self.current_screen_id.borrow());
        if let Some(screen) = screen {
            screen.on_set_screen(self);
        }
    }

    pub fn level_packs(&self) -> &RefCell<Vec<LevelPack>> {
        &self.level_packs
    }

    pub fn get_level_pack_count(&self) -> usize {
        self.level_packs.borrow().len()
    }

    pub fn get_level_pack_index(&self) -> usize {
        *self.current_level_pack_index.borrow()
    }

    pub fn set_level_pack_index(&self, level_pack_index: usize) {
        self.current_level_pack_index.replace(level_pack_index);
    }

    pub fn get_current_level_pack(&self) -> Option<Ref<LevelPack>> {
        let level_packs = self.level_packs.borrow();
        let index = self.get_level_pack_index();

        if level_packs.len() > index {
            return Some(Ref::map(level_packs, |level_packs| &level_packs[index]))
        }

        None
    }

    pub fn get_current_level_pack_mut(&self) -> Option<RefMut<LevelPack>> {
        let level_packs = self.level_packs.borrow_mut();
        let index = self.get_level_pack_index();

        if level_packs.len() > index {
            return Some(RefMut::map(level_packs, |level_packs| &mut level_packs[index]))
        }

        None
    }

    pub fn get_level_index(&self) -> usize {
        *self.current_level_index.borrow()
    }

    pub fn set_level_index(&self, level_index: usize) {
        self.current_level_index.replace(level_index);
    }

    pub fn is_player_background(&self) -> bool {
        *self.is_player_background.borrow()
    }

    pub fn open_help_page(&self) {
        self.is_help.replace(true);
    }

    pub fn close_help_page(&self) {
        self.is_help.replace(false);
    }

    pub fn is_dialog_opened(&self) -> bool {
        self.dialog.borrow().is_some()
    }

    pub fn open_dialog(&self, dialog: Dialog) {
        self.dialog.replace(Some(dialog));
    }

    pub fn close_dialog(&self) {
        self.dialog.replace(None);
    }

    pub fn exit(&self) {
        self.should_exit.replace(true);
    }
}

#[derive(Debug)]
pub struct GameError {
    message: String
}

impl GameError {
    fn new(message: impl Into<String>) -> Self {
        Self { message: message.into() }
    }
}

impl Display for GameError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl Error for GameError {}
