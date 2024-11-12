use console_lib::{keys, Console};
use std::collections::HashMap;
use std::error::Error;
use std::ffi::OsString;
use std::fmt::{Debug, Display, Formatter};
use std::mem;
use crate::game::help_page::HelpPage;
use crate::game::level::LevelPack;
use crate::game::screen::{Dialog, Screen, ScreenId, ScreenInGame, ScreenSelectLevel, ScreenSelectLevelPack, ScreenStartMenu};

mod level;
mod screen;
mod help_page;

struct GameState {
    current_screen_id: ScreenId,
    should_call_on_set_screen: bool,

    is_help: bool,
    dialog: Option<Dialog>,

    current_level_pack_index: usize,
    level_packs: Vec<LevelPack>,

    current_level_index: usize,

    is_player_background: bool,
    player_background_tmp: i32,

    should_exit: bool,
}

impl GameState {
    fn new(level_packs: Vec<LevelPack>) -> Self {
        Self {
            current_screen_id: ScreenId::StartMenu,
            should_call_on_set_screen: Default::default(),

            is_help: Default::default(),
            dialog: Default::default(),

            current_level_pack_index: Default::default(),
            level_packs,

            current_level_index: Default::default(),

            is_player_background: Default::default(),
            player_background_tmp: Default::default(),

            should_exit: Default::default(),
        }
    }

    pub fn set_screen(&mut self, screen_id: ScreenId) {
        self.current_screen_id = screen_id;
        self.should_call_on_set_screen = true;
    }

    pub fn level_packs(&self) -> &Vec<LevelPack> {
        &self.level_packs
    }

    pub fn get_level_pack_count(&self) -> usize {
        self.level_packs.len()
    }

    pub fn get_level_pack_index(&self) -> usize {
        self.current_level_pack_index
    }

    pub fn set_level_pack_index(&mut self, level_pack_index: usize) {
        self.current_level_pack_index = level_pack_index;
    }

    pub fn get_current_level_pack(&self) -> Option<&LevelPack> {
        self.level_packs.get(self.get_level_pack_index())
    }

    pub fn get_current_level_pack_mut(&mut self) -> Option<&mut LevelPack> {
        self.level_packs.get_mut(self.current_level_pack_index)
    }

    pub fn get_level_index(&self) -> usize {
        self.current_level_index
    }

    pub fn set_level_index(&mut self, level_index: usize) {
        self.current_level_index = level_index;
    }

    pub fn is_player_background(&self) -> bool {
        self.is_player_background
    }

    pub fn open_help_page(&mut self) {
        self.is_help = true;
    }

    pub fn close_help_page(&mut self) {
        self.is_help = false;
    }

    pub fn is_dialog_opened(&self) -> bool {
        self.dialog.is_some()
    }

    pub fn open_dialog(&mut self, dialog: Dialog) {
        self.dialog = Some(dialog);
    }

    pub fn close_dialog(&mut self) {
        self.dialog = None;
    }

    pub fn exit(&mut self) {
        self.should_exit = true;
    }
}

pub struct Game<'a> {
    console: &'a Console<'a>,

    screens: HashMap<ScreenId, Box<dyn Screen>>,
    help_page: HelpPage,

    game_state: GameState,
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
            help_page: HelpPage::new(),

            game_state: GameState::new(level_packs),
        })
    }

    #[must_use]
    pub fn update(&mut self) -> bool {
        if self.game_state.should_exit {
            return true;
        }

        if self.console.has_input() {
            self.update_key(self.console.get_key());
        }

        self.update_mouse();

        if !self.game_state.is_help {
            let screen = self.screens.get_mut(&self.game_state.current_screen_id);
            if let Some(screen) = screen {
                if mem::replace(&mut self.game_state.should_call_on_set_screen, false) {
                    screen.on_set_screen(&mut self.game_state);
                }

                screen.update(&mut self.game_state);
            }
        }

        //Player background
        self.game_state.player_background_tmp += 1;
        if self.game_state.player_background_tmp >= Self::PLAYER_BACKGROUND_DELAY + self.game_state.is_player_background as i32 {
            //If isPlayerBackground: wait an additional update (25 updates per second, every half
            //second: switch background/foreground colors [12 updates, 13 updates])
            self.game_state.player_background_tmp = 0;
            self.game_state.is_player_background = !self.game_state.is_player_background;
        }

        self.draw();

        false
    }

    fn update_key(&mut self, key: i32) {
        let screen = self.screens.get_mut(&mut self.game_state.current_screen_id);
        if self.game_state.is_help {
            if key == keys::F1 || key == keys::ESC {
                self.game_state.close_help_page();

                if let Some(screen) = screen {
                    screen.on_continue(&mut self.game_state);
                }
            }else {
                self.help_page.on_key_pressed(key);
            }

            return;
        }

        if let Some(screen) = screen {
            screen.on_key_pressed(&mut self.game_state, key);
        }
    }

    fn update_mouse(&mut self) {
        let (column, row) = self.console.get_mouse_pos_clicked();
        if column < 0 || row < 0 {
            return;
        }

        let (column, row) = (column as usize, row as usize);

        if self.game_state.is_help {
            self.help_page.on_mouse_pressed(Self::CONSOLE_MIN_WIDTH, Self::CONSOLE_MIN_HEIGHT, column, row);

            return;
        }

        let yes_no;
        if let Some(dialog) = self.game_state.dialog.as_ref() {
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

        let screen = self.screens.get_mut(&self.game_state.current_screen_id);
        if let Some(screen) = screen {
            screen.on_mouse_pressed(&mut self.game_state, column, row);
        }
    }

    fn draw(&self) {
        self.console.repaint();

        if self.game_state.is_help {
            self.help_page.draw(self.console, Self::CONSOLE_MIN_WIDTH, Self::CONSOLE_MIN_HEIGHT);

            return;
        }

        let screen = self.screens.get(&self.game_state.current_screen_id);
        if let Some(screen) = screen {
            screen.draw(&self.game_state, self.console);
        }

        if let Some(dialog) = self.game_state.dialog.as_ref() {
            dialog.draw(self.console, Self::CONSOLE_MIN_WIDTH, Self::CONSOLE_MIN_HEIGHT);
        }
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
