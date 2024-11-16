use console_lib::{keys, Console};
use std::collections::HashMap;
use std::error::Error;
use std::ffi::OsString;
use std::fmt::{Debug, Display, Formatter};
use std::mem;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use crate::game::help_page::HelpPage;
use crate::game::level::{Level, LevelPack, Tile};
use crate::game::screen::{Screen, ScreenId, ScreenInGame, ScreenLevelEditor, ScreenLevelPackEditor, ScreenSelectLevel, ScreenSelectLevelPack, ScreenSelectLevelPackEditor, ScreenStartMenu};
use crate::game::screen::dialog::Dialog;

mod level;
mod screen;
mod help_page;

struct EditorState {
    level_packs: Vec<LevelPack>,
    selected_level_pack_index: usize,
    selected_level_index: usize,
}

impl EditorState {
    pub fn new(level_packs: Vec<LevelPack>) -> Self {
        Self {
            level_packs,
            selected_level_pack_index: Default::default(),
            selected_level_index: Default::default(),
        }
    }

    pub fn get_level_pack_count(&self) -> usize {
        self.level_packs.len()
    }

    pub fn get_level_pack_index(&self) -> usize {
        self.selected_level_pack_index
    }

    pub fn get_current_level_pack(&self) -> Option<&LevelPack> {
        self.level_packs.get(self.selected_level_pack_index)
    }

    pub fn get_current_level_pack_mut(&mut self) -> Option<&mut LevelPack> {
        self.level_packs.get_mut(self.selected_level_pack_index)
    }

    pub fn set_level_pack_index(&mut self, level_pack_index: usize) {
        self.selected_level_pack_index = level_pack_index;
    }

    pub fn get_level_index(&self) -> usize {
        self.selected_level_index
    }

    pub fn set_level_index(&mut self, level_index: usize) {
        self.selected_level_index = level_index;
    }

    pub fn get_current_level(&self) -> Option<&Level> {
        self.level_packs.get(self.selected_level_pack_index).
                and_then(|level_pack| level_pack.levels().get(self.selected_level_index)).
                map(|level_with_stats| level_with_stats.level())
    }

    pub fn get_current_level_mut(&mut self) -> Option<&mut Level> {
        self.level_packs.get_mut(self.selected_level_pack_index).
                and_then(|level_pack| level_pack.levels_mut().get_mut(self.selected_level_index)).
                map(|level_with_stats| level_with_stats.level_mut())
    }
}

struct GameState {
    current_screen_id: ScreenId,
    should_call_on_set_screen: bool,

    is_help: bool,
    dialog: Option<Box<dyn Dialog>>,

    current_level_pack_index: usize,
    level_packs: Vec<LevelPack>,

    current_level_index: usize,

    is_player_background: bool,
    player_background_tmp: i32,

    found_secret_main_level_pack: bool,

    should_exit: bool,

    editor_state: EditorState,
}

impl GameState {
    fn new(level_packs: Vec<LevelPack>, editor_level_packs: Vec<LevelPack>) -> Self {
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

            found_secret_main_level_pack: Default::default(),

            should_exit: Default::default(),

            editor_state: EditorState::new(editor_level_packs),
        }
    }

    pub fn set_screen(&mut self, screen_id: ScreenId) {
        self.current_screen_id = screen_id;
        self.should_call_on_set_screen = true;
    }

    pub fn level_packs(&self) -> &[LevelPack] {
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
        self.level_packs.get(self.current_level_pack_index)
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

    pub fn open_dialog(&mut self, dialog: Box<dyn Dialog>) {
        self.dialog = Some(dialog);
    }

    pub fn close_dialog(&mut self) {
        self.dialog = None;
    }

    pub fn exit(&mut self) {
        self.should_exit = true;
    }

    fn on_found_secret_for_level_pack(&mut self, level_pack_index: usize) -> Result<(), Box<dyn Error>> {
        if level_pack_index == 1 && !self.found_secret_main_level_pack {
            self.found_secret_main_level_pack = true;

            let secret_level_pack = LevelPack::read_from_save_game("secret", "build-in:secret", Game::MAP_SECRET)?;

            //Save immediately in order to keep secret level pack after game restart if not yet played
            secret_level_pack.save_save_game()?;

            self.level_packs.insert(4, secret_level_pack);
        }

        Ok(())
    }

    pub fn on_found_secret(&mut self) -> Result<(), Box<dyn Error>> {
        self.on_found_secret_for_level_pack(self.current_level_pack_index)
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

    pub const LEVEL_MAX_WIDTH: usize = Self::CONSOLE_MIN_WIDTH;
    pub const LEVEL_MAX_HEIGHT: usize = Self::CONSOLE_MIN_HEIGHT - 1;

    pub const MAX_LEVEL_PACK_ID_LEN: usize = 16;

    const PLAYER_BACKGROUND_DELAY: i32 = 12;

    const SAVE_GAME_FOLDER: &'static str = "ConsoleSokoban";

    const MAP_TUTORIAL: &'static str = include_str!("../resources/tutorial.lvl");
    const MAP_MAIN: &'static str = include_str!("../resources/main.lvl");
    const MAP_SPECIAL: &'static str = include_str!("../resources/special.lvl");
    const MAP_DEMON: &'static str = include_str!("../resources/demon.lvl");

    const MAP_SECRET: &'static str = include_str!("../resources/secret.lvl");

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

            (ScreenId::SelectLevelPackEditor, Box::new(ScreenSelectLevelPackEditor::new()) as Box<dyn Screen>),
            (ScreenId::LevelPackEditor, Box::new(ScreenLevelPackEditor::new()) as Box<dyn Screen>),
            (ScreenId::LevelEditor, Box::new(ScreenLevelEditor::new()) as Box<dyn Screen>),
        ]);

        let mut level_packs = Vec::with_capacity(LevelPack::MAX_LEVEL_PACK_COUNT);
        level_packs.append(&mut vec![
            LevelPack::read_from_save_game("tutorial", "build-in:tutorial", Self::MAP_TUTORIAL)?,
            LevelPack::read_from_save_game("main", "build-in:main", Self::MAP_MAIN)?,
            LevelPack::read_from_save_game("special", "build-in:special", Self::MAP_SPECIAL)?,
            LevelPack::read_from_save_game("demon", "build-in:demon", Self::MAP_DEMON)?,
        ]);

        for arg in std::env::args().
                skip(1) {
            if !arg.ends_with(".lvl") {
                return Err(Box::new(GameError::new(format!(
                    "Invalid level pack \"{}\": The file extension of level pack must be \".lvl\"",
                    arg
                ))));
            }

            let level_pack_path = Path::new(&arg);

            let level_pack_file_name = if let Some(file_name) = level_pack_path.file_name() {
                if let Some(file_name) = file_name.to_str() {
                    file_name
                }else {
                    return Err(Box::new(GameError::new(format!(
                        "Error while loading level pack \"{}\": Invalid file name",
                        arg
                    ))));
                }
            }else {
                return Err(Box::new(GameError::new(format!(
                    "Error while loading level pack \"{}\": File name is missing",
                    arg
                ))));
            };

            let mut level_pack_file = match File::open(level_pack_path) {
                Ok(file) => file,
                Err(err) => return Err(Box::new(GameError::new(format!(
                    "Error while loading level pack \"{}\": {}",
                    arg, err
                )))),
            };

            let mut level_pack_data = String::new();
            if let Err(err) = level_pack_file.read_to_string(&mut level_pack_data) {
                return Err(Box::new(GameError::new(format!(
                    "Error while loading level pack \"{}\": {}",
                    arg, err
                ))));
            };

            let level_pack_id = &level_pack_file_name[..level_pack_file_name.len() - 4];
            if level_pack_id.len() > Self::MAX_LEVEL_PACK_ID_LEN {
                return Err(Box::new(GameError::new(format!(
                    "Error while loading level pack \"{}\": Level pack ID is too long (Max: {})",
                    arg, Self::MAX_LEVEL_PACK_ID_LEN
                ))));
            }

            level_packs.push(LevelPack::read_from_save_game(level_pack_id, &arg, level_pack_data)?);
        }

        if level_packs.len() > LevelPack::MAX_LEVEL_PACK_COUNT {
            return Err(Box::new(GameError::new(format!(
                "Too many level packs ({}, max: {})",
                level_packs.len(),
                LevelPack::MAX_LEVEL_PACK_COUNT,
            ))));
        }

        for level_pack in level_packs.iter() {
            if level_pack.level_count() == 0 {
                return Err(Box::new(GameError::new(format!(
                    "Error while loading level pack \"{}\": Level pack contains no levels",
                    level_pack.id()
                ))));
            }

            if level_pack.level_count() > LevelPack::MAX_LEVEL_COUNT_PER_PACK {
                return Err(Box::new(GameError::new(format!(
                    "Error while loading level pack \"{}\": Level pack contains too many levels ({}, max: {})",
                    level_pack.id(),
                    level_pack.level_count(),
                    LevelPack::MAX_LEVEL_COUNT_PER_PACK,
                ))));
            }

            for (i, level) in level_pack.levels().iter().
                    map(|level| level.level()).
                    enumerate() {
                if level.width() > Self::LEVEL_MAX_WIDTH || level.height() > Self::LEVEL_MAX_HEIGHT {
                    return Err(Box::new(GameError::new(format!(
                        "Error while loading level pack \"{}\": Level {} is too large (Max: {}x{})",
                        level_pack.id(),
                        i + 1,
                        Self::LEVEL_MAX_WIDTH,
                        Self::LEVEL_MAX_HEIGHT,
                    ))));
                }

                let player_tile_count = level.tiles().iter().filter(|tile| **tile == Tile::Player).count();
                if player_tile_count == 0 {
                    return Err(Box::new(GameError::new(format!(
                        "Error while loading level pack \"{}\": Level {} does not contain a player tile",
                        level_pack.id(),
                        i + 1,
                    ))));
                }else if player_tile_count > 1 {
                    return Err(Box::new(GameError::new(format!(
                        "Error while loading level pack \"{}\": Level {} contains too many player tiles",
                        level_pack.id(),
                        i + 1,
                    ))));
                }
            }
        }

        let mut editor_level_packs = Vec::with_capacity(LevelPack::MAX_LEVEL_PACK_COUNT);

        let save_game_folder = Game::get_or_create_save_game_folder()?;
        for entry in std::fs::read_dir(save_game_folder)?.
                filter(|entry| entry.as_ref().
                        is_ok_and(|entry| entry.path().is_file())).
                map(|entry| entry.unwrap()) {
            if entry.file_name().to_str().is_some_and(|file_name| file_name.ends_with(".lvl.edit")) {
                let file_name = entry.file_name();
                let file_name = file_name.to_str().unwrap();
                let level_pack_id = &file_name[..file_name.len() - 9];

                let mut level_pack_file = match File::open(entry.path()) {
                    Ok(file) => file,
                    Err(err) => return Err(Box::new(GameError::new(format!(
                        "Error while loading editor level pack \"{}\": {}",
                        file_name, err
                    )))),
                };

                let mut level_pack_data = String::new();
                if let Err(err) = level_pack_file.read_to_string(&mut level_pack_data) {
                    return Err(Box::new(GameError::new(format!(
                        "Error while loading editor level pack \"{}\": {}",
                        file_name, err
                    ))));
                };

                editor_level_packs.push(LevelPack::read_from_save_game(level_pack_id, entry.path().to_str().unwrap(), level_pack_data)?);
            }
        }

        if editor_level_packs.len() > LevelPack::MAX_LEVEL_PACK_COUNT {
            return Err(Box::new(GameError::new(format!(
                "Too many level packs ({}, max: {})",
                editor_level_packs.len(),
                LevelPack::MAX_LEVEL_PACK_COUNT,
            ))));
        }

        for level_pack in editor_level_packs.iter() {
            //Level pack for editor might be empty and might contain no player tile

            if level_pack.level_count() > LevelPack::MAX_LEVEL_COUNT_PER_PACK {
                return Err(Box::new(GameError::new(format!(
                    "Error while loading editor level pack \"{}\": Level pack contains too many levels ({}, max: {})",
                    level_pack.id(),
                    level_pack.level_count(),
                    LevelPack::MAX_LEVEL_COUNT_PER_PACK,
                ))));
            }

            for (i, level) in level_pack.levels().iter().
                    map(|level| level.level()).
                    enumerate() {
                if level.width() > Self::LEVEL_MAX_WIDTH || level.height() > Self::LEVEL_MAX_HEIGHT {
                    return Err(Box::new(GameError::new(format!(
                        "Error while loading editor level pack \"{}\": Level {} is too large (Max: {}x{})",
                        level_pack.id(),
                        i + 1,
                        Self::LEVEL_MAX_WIDTH,
                        Self::LEVEL_MAX_HEIGHT,
                    ))));
                }
            }
        }
        
        editor_level_packs.sort_by_key(|level_pack| level_pack.id().to_string());

        let mut game_state = GameState::new(level_packs, editor_level_packs);

        let mut save_game_file = Game::get_or_create_save_game_folder()?;
        save_game_file.push("secret.lvl.sav");
        if std::fs::exists(&save_game_file).is_ok_and(|exists| exists) {
            game_state.on_found_secret_for_level_pack(1)?;
        }

        Ok(Self {
            console,

            screens,
            help_page: HelpPage::new(),

            game_state,
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
        let screen = self.screens.get_mut(&self.game_state.current_screen_id);
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

        if let Some(dialog) = self.game_state.dialog.as_ref() {
            if let Some(dialog_selection) = dialog.on_key_pressed(Self::CONSOLE_MIN_WIDTH, Self::CONSOLE_MIN_HEIGHT, key) {
                self.game_state.close_dialog();

                let screen = self.screens.get_mut(&self.game_state.current_screen_id);
                if let Some(screen) = screen {
                    screen.on_dialog_selection(&mut self.game_state, dialog_selection);
                }
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

        if let Some(dialog) = self.game_state.dialog.as_ref() {
            if let Some(dialog_selection) = dialog.on_mouse_pressed(Self::CONSOLE_MIN_WIDTH, Self::CONSOLE_MIN_HEIGHT, column, row) {
                self.game_state.close_dialog();

                let screen = self.screens.get_mut(&self.game_state.current_screen_id);
                if let Some(screen) = screen {
                    screen.on_dialog_selection(&mut self.game_state, dialog_selection);
                }
            }

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
