use console_lib::{keys, Color, Console};

enum SectionLayer {
    Section(u32),
    SubSection(u32, u32),
    SubSubSection(u32, u32, u32),
}

impl SectionLayer {
    pub fn get_heading_color(&self) -> Color {
        match self {
            Self::Section(_) => Color::Blue,
            Self::SubSection(_, _) => Color::Green,
            Self::SubSubSection(_, _, _) => Color::Cyan,
        }
    }
}

struct Section {
    layer: SectionLayer,
    name: String,
    page: u32,
}

impl Section {
    pub fn new(layer: SectionLayer, name: impl Into<String>, page: u32) -> Self {
        Self { layer, name: name.into(), page }
    }

    pub fn draw(&self, console: &Console, width: usize) {
        console.set_color(self.layer.get_heading_color(), Color::Default);

        let heading = match self.layer {
            SectionLayer::Section(section) => {
                format!("{} {}", section, self.name)
            },
            SectionLayer::SubSection(section, sub_section) => {
                format!("  {}.{} {}", section, sub_section, self.name)
            },
            SectionLayer::SubSubSection(section, sub_section, sub_sub_section) => {
                format!("      {}.{}.{} {}", section, sub_section, sub_sub_section, self.name)
            },
        };
        let heading_len = heading.chars().count();

        let page = (self.page + 1).to_string();
        let page_len = page.chars().count();

        console.draw_text(format!("{}{}{}", heading, ".".repeat(width - heading_len - page_len), page));
    }
}

struct TableOfContents {
    next_section: u32,
    next_sub_section: u32,
    next_sub_sub_section: u32,

    sections: Vec<Section>,
}

impl TableOfContents {
    pub fn new() -> Self {
        Self {
            next_section: Default::default(),
            next_sub_section: Default::default(),
            next_sub_sub_section: Default::default(),

            sections: Vec::new(),
        }
    }

    pub fn add_section(&mut self, name: impl Into<String>, page: u32) {
        self.next_section += 1;
        self.next_sub_section = 0;
        self.next_sub_sub_section = 0;

        self.sections.push(Section::new(
            SectionLayer::Section(
                self.next_section
            ), name, page
        ));
    }

    pub fn add_sub_section(&mut self, name: impl Into<String>, page: u32) {
        self.next_sub_section += 1;
        self.next_sub_sub_section = 0;

        self.sections.push(Section::new(
            SectionLayer::SubSection(
                self.next_section,
                self.next_sub_section
            ), name, page
        ));
    }

    pub fn add_sub_sub_section(&mut self, name: impl Into<String>, page: u32) {
        self.next_sub_sub_section += 1;

        self.sections.push(Section::new(
            SectionLayer::SubSubSection(
                self.next_section,
                self.next_sub_section,
                self.next_sub_sub_section
            ), name, page
        ));
    }

    pub fn draw(&self, console: &Console, x: usize, y: usize, width: usize, height: usize, page: u32) {
        for (i, section) in self.sections.iter().
                skip(height * page as usize).
                take(height).
                enumerate() {
            console.set_cursor_pos(x, y + i);
            section.draw(console, width);
        }

        console.reset_color();
    }

    pub fn get_page_mouse_clicked(&self, height: usize, page: u32, row: u32) -> Option<u32> {
        self.sections.get(height * page as usize + row as usize).map(|section| section.page)
    }
}

pub struct HelpPage {
    table_of_contents: TableOfContents,

    page: u32,
}

impl HelpPage {
    const PAGE_COUNT: u32 = 6;

    pub fn new() -> Self {
        let mut table_of_contents = TableOfContents::new();
        table_of_contents.add_section("Control", 1);
        table_of_contents.add_sub_section("Keyboard", 1);
        table_of_contents.add_sub_sub_section("Help menu", 1);
        table_of_contents.add_sub_sub_section("Exit window", 1);
        table_of_contents.add_sub_sub_section("Start menu", 1);
        table_of_contents.add_sub_sub_section("Game control", 1);
        table_of_contents.add_sub_section("Mouse input", 2);
        table_of_contents.add_sub_sub_section("Help menu", 2);
        table_of_contents.add_sub_sub_section("Exit window", 2);
        table_of_contents.add_sub_sub_section("Start menu", 3);
        table_of_contents.add_section("Console arguments", 3);
        table_of_contents.add_section("Menus", 4);
        table_of_contents.add_sub_section("Help menu", 4);
        table_of_contents.add_sub_section("Exit window", 4);
        table_of_contents.add_sub_section("Game screen", 4);
        table_of_contents.add_section("Gameplay", 5);
        table_of_contents.add_sub_section("Play", 5);
        table_of_contents.add_sub_section("Game over", 5);

        Self {
            table_of_contents,
            page: Default::default(),
        }
    }

    pub fn draw(&self, console: &Console, width: usize, height: usize) {
        console.set_color(Color::Yellow, Color::Default);
        console.set_underline(true);
        console.draw_text("Help menu");

        console.set_cursor_pos(0, 2);
        match self.page {
            page @ 0 => {
                console.set_underline(false);
                self.table_of_contents.draw(console, 0, 2, width, height - 4, page);
            },
            1 => {
                console.set_color(Color::Blue, Color::Default);
                console.draw_text("1 Control\n");

                console.set_color(Color::Green, Color::Default);
                console.draw_text("1.1 Keyboard\n");

                console.set_underline(false);
                console.set_color(Color::LightRed, Color::Default);
                console.draw_text("F1");
                console.reset_color();
                console.draw_text(": Open help menu");

                console.set_underline(true);
                console.set_color(Color::Cyan, Color::Default);
                console.set_cursor_pos(0, 6);
                console.draw_text("1.1.1 Help menu\n");

                console.set_underline(false);
                console.set_color(Color::LightRed, Color::Default);
                console.draw_text("ESC");
                console.reset_color();
                console.draw_text("/");
                console.set_color(Color::LightRed, Color::Default);
                console.draw_text("F1");
                console.reset_color();
                console.draw_text(": Exit help menu\n");

                console.set_color(Color::LightRed, Color::Default);
                console.draw_text("UP");
                console.reset_color();
                console.draw_text("/");
                console.set_color(Color::LightRed, Color::Default);
                console.draw_text("DOWN");
                console.reset_color();
                console.draw_text(": Switch page");

                console.set_underline(true);
                console.set_color(Color::Cyan, Color::Default);
                console.set_cursor_pos(0, 10);
                console.draw_text("1.1.2 Exit window\n");

                console.set_underline(false);
                console.set_color(Color::LightRed, Color::Default);
                console.draw_text("y");
                console.reset_color();
                console.draw_text("/");
                console.set_color(Color::LightRed, Color::Default);
                console.draw_text("n");
                console.reset_color();
                console.draw_text(": Yes (Exit)/No (Not exit)");

                console.set_underline(true);
                console.set_color(Color::Cyan, Color::Default);
                console.set_cursor_pos(0, 13);
                console.draw_text("1.1.3 Start menu\n");

                console.set_underline(false);
                console.set_color(Color::LightRed, Color::Default);
                console.draw_text("ENTER");
                console.reset_color();
                console.draw_text(": Start game/Next Level\n");

                console.set_color(Color::LightRed, Color::Default);
                console.draw_text("ESC");
                console.reset_color();
                console.draw_text(": Exit window");

                console.set_underline(true);
                console.set_color(Color::Cyan, Color::Default);
                console.set_cursor_pos(0, 17);
                console.draw_text("1.1.4 Game control\n");

                console.set_underline(false);
                console.set_color(Color::LightRed, Color::Default);
                console.draw_text("Arrow keys");
                console.reset_color();
                console.draw_text(": Move position\n");
                console.set_color(Color::LightRed, Color::Default);
                console.draw_text("r");
                console.reset_color();
                console.draw_text(": Reset level\n");
                console.set_underline(false);
                console.set_color(Color::LightRed, Color::Default);
                console.draw_text("z");
                console.reset_color();
                console.draw_text(": One step back/forward");
            },
            2 => {
                console.set_color(Color::Green, Color::Default);
                console.draw_text("1.2 Mouse input\n");

                console.set_underline(false);
                console.reset_color();
                console.draw_text("Left click: [");
                console.set_color(Color::Default, Color::Yellow);
                console.draw_text("L");
                console.reset_color();
                console.draw_text("] \"Position\"\n");
                console.draw_text("Right click: [");
                console.set_color(Color::Default, Color::Yellow);
                console.draw_text("R");
                console.reset_color();
                console.draw_text("] \"Position\"\n");
                console.draw_text("Middle click: [");
                console.set_color(Color::Default, Color::Yellow);
                console.draw_text("M");
                console.reset_color();
                console.draw_text("] \"Position\"");

                console.set_underline(true);
                console.set_color(Color::Cyan, Color::Default);
                console.set_cursor_pos(0, 7);
                console.draw_text("1.2.1 Help menu\n");

                console.set_underline(false);
                console.reset_color();
                console.draw_text("[");
                console.set_color(Color::Default, Color::Yellow);
                console.draw_text("L");
                console.reset_color();
                console.draw_text("] \"Page: 00\": Switch page (The same as ");
                console.set_color(Color::LightRed, Color::Default);
                console.draw_text("DOWN");
                console.reset_color();
                console.draw_text(")\n[");
                console.set_color(Color::Default, Color::Yellow);
                console.draw_text("L");
                console.reset_color();
                console.draw_text("] Chapter at first pages: Goto page");

                console.set_underline(true);
                console.set_color(Color::Cyan, Color::Default);
                console.set_cursor_pos(0, 11);
                console.draw_text("1.2.2 Exit window\n");

                console.set_underline(false);
                console.reset_color();
                console.draw_text("[");
                console.set_color(Color::Default, Color::Yellow);
                console.draw_text("L");
                console.reset_color();
                console.draw_text("] \"[y]es\": Yes (The same as ");
                console.set_color(Color::LightRed, Color::Default);
                console.draw_text("y");
                console.reset_color();
                console.draw_text(")\n[");
                console.set_color(Color::Default, Color::Yellow);
                console.draw_text("L");
                console.reset_color();
                console.draw_text("] \"[n]o\": No (The same as ");
                console.set_color(Color::LightRed, Color::Default);
                console.draw_text("n");
                console.reset_color();
                console.draw_text(")");

                console.set_underline(true);
                console.set_color(Color::Cyan, Color::Default);
                console.set_cursor_pos(0, 15);
                console.draw_text("1.2.3 Start menu\n");

                console.set_underline(false);
                console.reset_color();
                console.draw_text("[");
                console.set_color(Color::Default, Color::Yellow);
                console.draw_text("L");
                console.reset_color();
                console.draw_text("] \"ENTER\": Start game (The same as ");
                console.set_color(Color::LightRed, Color::Default);
                console.draw_text("ENTER");
                console.reset_color();
                console.draw_text(")\n[");
                console.set_color(Color::Default, Color::Yellow);
                console.draw_text("L");
                console.reset_color();
                console.draw_text("] \"Help: F1\": Open help menu (The same as ");
                console.set_color(Color::LightRed, Color::Default);
                console.draw_text("F1");
                console.reset_color();
                console.draw_text(")");
            },
            3 => {
                console.set_color(Color::Blue, Color::Default);
                console.draw_text("2 Console arguments\n");

                console.set_underline(false);
                console.reset_color();
                console.draw_text("1) No arguments\n2) \"Path to level pack 1\" \"Path to level pack 2\" ...");
            },
            4 => {
                console.set_color(Color::Blue, Color::Default);
                console.draw_text("3 Menus\n");
                console.set_color(Color::Green, Color::Default);
                console.draw_text("3.1 Help menu\n");

                console.set_underline(false);
                console.reset_color();
                console.draw_text("\"Page: x of y\": x: (Current page), y: (Last page)\n\"");
                console.set_color(Color::Blue, Color::Default);
                console.set_underline(true);
                console.draw_text("x Title");
                console.set_underline(false);
                console.reset_color();
                console.draw_text("\":     Heading 1 (Chapter Name)\n\"");
                console.set_color(Color::Green, Color::Default);
                console.set_underline(true);
                console.draw_text("x.x Title");
                console.set_underline(false);
                console.reset_color();
                console.draw_text("\":   Heading 2 (Chapter.Chapter Name)\n\"");
                console.set_color(Color::Cyan, Color::Default);
                console.set_underline(true);
                console.draw_text("x.x.x Title");
                console.set_underline(false);
                console.reset_color();
                console.draw_text("\": Heading 3 (Chapter.Chapter.Chapter Name)\n");

                console.set_underline(true);
                console.set_color(Color::Green, Color::Default);
                console.set_cursor_pos(0, 9);
                console.draw_text("3.2 Exit window\n");

                console.set_underline(false);
                console.reset_color();
                console.draw_text("Confirm exit\n");

                console.set_underline(true);
                console.set_color(Color::Green, Color::Default);
                console.set_cursor_pos(0, 12);
                console.draw_text("3.3 Game screen\n");

                console.set_underline(false);

                console.reset_color();
                console.set_cursor_pos(1, 13);
                console.draw_text(
                    ": Empty\n       : One way doors\n : Wall\n : Player\n   : Box\n \
                    : Goal\n   : Key\n : Locked Door"
                );

                console.set_color(Color::LightBlue, Color::Default);
                console.set_cursor_pos(0, 13);
                console.draw_text("-\n< ^ > v");
                console.set_color(Color::LightGreen, Color::Default);
                console.set_cursor_pos(0, 15);
                console.draw_text("#");
                console.set_color(Color::Yellow, Color::Default);
                console.set_cursor_pos(0, 16);
                console.draw_text("P");
                console.set_color(Color::LightCyan, Color::Default);
                console.set_cursor_pos(0, 17);
                console.draw_text("@");
                console.set_color(Color::Pink, Color::Default);
                console.set_cursor_pos(2, 17);
                console.draw_text("@");
                console.set_color(Color::LightRed, Color::Default);
                console.set_cursor_pos(0, 18);
                console.draw_text("x");
                console.set_color(Color::LightCyan, Color::Default);
                console.set_cursor_pos(0, 19);
                console.draw_text("*");
                console.set_color(Color::Pink, Color::Default);
                console.set_cursor_pos(2, 19);
                console.draw_text("*");
                console.set_color(Color::LightRed, Color::Default);
                console.set_cursor_pos(0, 20);
                console.draw_text("=");
            },
            5 => {
                console.set_color(Color::Blue, Color::Default);
                console.draw_text("4 Gameplay\n");
                console.set_color(Color::Green, Color::Default);
                console.draw_text("4.1 Game\n");

                console.set_underline(false);
                console.reset_color();
                console.draw_text("Move all boxes to the goals.");

                console.set_underline(true);
                console.set_color(Color::Green, Color::Default);
                console.set_cursor_pos(0, 6);
                console.draw_text("4.2 Game over\n");

                console.set_underline(false);
                console.reset_color();
                console.draw_text("Press ");
                console.set_color(Color::LightRed, Color::Default);
                console.draw_text("ESC");
                console.reset_color();
                console.draw_text(" to left the game (See: ");
                console.set_color(Color::Green, Color::Default);
                console.draw_text("1.1 Keyboard");
                console.reset_color();
                console.draw_text(").");
            },
            _ => {},
        }

        console.set_cursor_pos(0, height - 1);
        console.reset_color();
        console.draw_text("Page: ");
        console.set_color(Color::Cyan, Color::Default);
        console.draw_text(format!("{}", self.page + 1));
        console.reset_color();
        console.draw_text(" of ");
        console.set_color(Color::Cyan, Color::Default);
        console.draw_text(format!("{}", Self::PAGE_COUNT));
    }

    pub fn on_key_pressed(&mut self, key: i32) {
        if key == keys::UP {
            self.page = if self.page == 0 {
                Self::PAGE_COUNT - 1
            }else {
                self.page - 1
            };
        }else if key == keys::DOWN {
            self.page = if self.page == Self::PAGE_COUNT - 1 {
                0
            }else {
                self.page + 1
            };
        }
    }

    pub fn on_mouse_pressed(&mut self, _width: usize, height: usize, column: usize, row: usize) {
        if row > 2 && row < height - 2 {
            if let Some(page_clicked) = self.table_of_contents.get_page_mouse_clicked(height, self.page, row as u32 - 2) {
                self.page = page_clicked;
            }
        }

        if row == height - 1 && column < 8 {
            self.on_key_pressed(keys::DOWN);
        }
    }
}
