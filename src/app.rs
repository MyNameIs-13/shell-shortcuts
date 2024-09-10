use std::collections::HashMap;
use crate::fl;
use cosmic::app::{Command, Core};
use cosmic::iced::alignment::{Horizontal, Vertical};
use cosmic::iced::{Alignment, Length, Padding};
use cosmic::{cosmic_theme, theme, Application, ApplicationExt, Element, Renderer, Theme};
use cosmic::widget::{self, menu, text, button, Text};

const REPOSITORY: &str = "https://github.com/MyNameIs-13/shell-shortcuts";

/// This is the struct that represents your application.
/// It is used to define the data that will be used by your application.
pub struct PopShortcuts {
    /// Application state which is managed by the COSMIC runtime.
    core: Core,
    /// Display a context drawer with the designated page if defined.
    context_page: ContextPage,
    /// Key bindings for the application's menu bar.
    key_binds: HashMap<menu::KeyBind, MenuAction>,
}

/// This is the enum that contains all the possible variants that your application will need to transmit messages.
/// This is used to communicate between the different parts of your application.
/// If your application does not need to send messages, you can use an empty enum or `()`.
#[derive(Debug, Clone)]
pub enum Message {
    LaunchUrl(String),
    ToggleContextPage(ContextPage),
}

/// Identifies a context page to display in the context drawer.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum ContextPage {
    #[default]
    About,
}

impl ContextPage {
    fn title(&self) -> String {
        match self {
            Self::About => fl!("about"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MenuAction {
    About,
}

impl menu::action::MenuAction for MenuAction {
    type Message = Message;
    fn message(&self) -> Self::Message {
        match self {
            MenuAction::About => Message::ToggleContextPage(ContextPage::About),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Shortcut {
    description: String,
    keys: Vec<String>,
}

impl Shortcut {
    pub fn new(description: &str, keys: &[&str]) -> Self {
        Self {
            description: description.to_string(),
            keys: keys.iter().map(|&key| key.to_string()).collect(),
        }
    }

    pub fn display_keys(&self) -> Vec<String> {
        self.keys.clone()
    }
}

fn centralize_tile_content(tile_content: Text<Theme, Renderer>) -> Text<Theme, Renderer> {
    tile_content
        .horizontal_alignment(Horizontal::Center)
        .vertical_alignment(Vertical::Center)
}

fn get_max_keys() -> f32 {
    let shortcuts = get_shortcuts(); // Assuming get_shortcuts() is defined elsewhere
    let mut max_keys = 0.0;
    for category in shortcuts {
        for shortcut in category.shortcuts {
            let num_keys = shortcut.keys.len() as f32;
            if num_keys > max_keys {
                max_keys = num_keys;
            }
        }
    }
    max_keys
}

// Render the keys as buttons with the keycap style
fn render_keys(keys: Vec<String>) -> Element<'static, Message> {
    let mut row_of_keys = widget::Row::new().spacing(5); // Adjust spacing between keycaps

    for key in keys {
        let keycap = button(centralize_tile_content(text(key)))
            .padding(Padding {
                top: 0.0,
                bottom: 0.0,
                left: 15.0,
                right: 15.0,
            })   // Padding inside the button
            .height(Length::Fixed(30.0))
            .style(theme::Button::MenuItem); // Ensure this is styled appropriately
        row_of_keys = row_of_keys.push(keycap);
    }

    widget::Container::new(row_of_keys)
        .width(Length::Fixed(60.0*get_max_keys())) // Set fixed width here
        .padding(5) // Optional paddingc
        .center_y() // Optional alignment
        .into()
}

#[derive(Clone, Debug)]
pub struct Category {
    name: String,
    shortcuts: Vec<Shortcut>,
}

impl Category {
    pub fn new(name: &str, shortcuts: Vec<Shortcut>) -> Self {
        Self {
            name: name.to_string(),
            shortcuts,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn shortcuts(&self) -> &Vec<Shortcut> {
        &self.shortcuts
    }
}

fn get_shortcuts() -> Vec<Category> {
    vec![
        Category::new("Launcher Shortcuts", vec![
            Shortcut::new("Activate Launcher", &["Super"]),
            Shortcut::new("Scroll through the Launcher list", &["↑", "↓"]),
            Shortcut::new("Execute a command in a terminal", &["t:"]),
            Shortcut::new("Execute a command in sh", &[":"]),
            Shortcut::new("Calculate an equation", &["="]),
            Shortcut::new("Show all Applications", &["Super", "A"]),
            Shortcut::new("Launch Terminal", &["Super", "T"]),
            Shortcut::new("Launch Browser", &["Super", "B"]),
            Shortcut::new("Launch File Manager", &["Super", "E"]),
            Shortcut::new("Launch Text Editor", &["Super", "F"]),
            Shortcut::new("Launch this overview", &["Super", "Z"]),
        ]),
        Category::new("Window & Workspace Management Shortcuts", vec![
            Shortcut::new("Show Workspace overview", &["Super", "Tab"]),
            Shortcut::new("Switch between open windows", &["Alt", "Tab"]),
            Shortcut::new("Switch focus to the workspace above", &["Ctrl", "Super", "↑"]),
            Shortcut::new("Switch focus to the workspace below", &["Ctrl", "Super", "↓"]),
            Shortcut::new("Switch focus between windows", &["Super", "←", "↓", "↑", "→"]),
            Shortcut::new("Move window", &["Super", "Shift", "←", "↓", "↑", "→"]),
            Shortcut::new("Move window up one workspace", &["Ctrl", "Super", "Shift", "↑"]),
            Shortcut::new("Move window down one workspace", &["Ctrl", "Super", "Shift", "↓"]),
            Shortcut::new("Move window one monitor to the left", &["Ctrl", "Super", "Shift", "←"]),
            Shortcut::new("Move window to one monitor to the right", &["Ctrl", "Super", "Shift", "→"]),
            Shortcut::new("Swap windows (keep pressed, then use arrow keys)", &["Super", "X"]),
            Shortcut::new("Toggle window orientation", &["Super", "O"]),
            Shortcut::new("Toggle floating mode", &["Super", "G"]),
            Shortcut::new("Toggle auto-tiling", &["Super", "Y"]),
            Shortcut::new("Toggle stacking mode", &["Super", "S"]),
            Shortcut::new("Toggle maximize", &["Super", "M"]),
            Shortcut::new("Close window", &["Super", "Q"]),
            Shortcut::new("Hide current window", &["Super", "H"]),
            Shortcut::new("Resize window inward (smaller)", &["Super", "Shift", "R"]),
            Shortcut::new("Resize window outward (bigger)", &["Super", "R"]),
        ]),
    ]
}

fn render_shortcuts() -> Element<'static, Message> {
    let categories = get_shortcuts();

    let mut category_elements = widget::column()
        .spacing(30) // Space between categories
        .padding(20); // Padding around the entire column

    for category in categories {
        let mut category_column = widget::column()
            .push(widget::text::title2(category.name().to_string())
                .size(18))
            .spacing(5);

        for shortcut in category.shortcuts() {
            let keys = render_keys(shortcut.display_keys());

            let desc_text = text(shortcut.description.to_string());

            let row = widget::row()
                .push(keys)
                .push(desc_text)
                .align_items(Alignment::Center); // Align items within the row to the start

            category_column = category_column.push(row);
        }
        category_elements = category_elements.push(category_column);
    }
    category_elements
        .spacing(20) // Space between rows within a category
        .into()
}

/// Implement the `Application` trait for your application.
/// This is where you define the behavior of your application.
///
/// The `Application` trait requires you to define the following types and constants:
/// - `Executor` is the async executor that will be used to run your application's commands.
/// - `Flags` is the data that your application needs to use before it starts.
/// - `Message` is the enum that contains all the possible variants that your application will need to transmit messages.
/// - `APP_ID` is the unique identifier of your application.
impl Application for PopShortcuts {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;
    const APP_ID: &'static str = "com.github.MyNameIs-13.CosmicPopShortcuts";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    /// This is the entry point of your application, it is where you initialize your application.
    ///
    /// Any work that needs to be done before the application starts should be done here.
    ///
    /// - `core` is used to passed on for you by libcosmic to use in the core of your own application.
    /// - `flags` is used to pass in any data that your application needs to use before it starts.
    /// - `Command` type is used to send messages to your application. `Command::none()` can be used to send no messages to your application.
    fn init(core: Core, _flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let mut app = PopShortcuts {
            core,
            context_page: ContextPage::default(),
            key_binds: HashMap::new(),
        };
        let command = app.update_titles();
        (app, command)
    }

    /// Elements to pack at the start of the header bar.
    fn header_start(&self) -> Vec<Element<Self::Message>> {
        let menu_bar = menu::bar(vec![menu::Tree::with_children(
            menu::root(fl!("view")),
            menu::items(
                &self.key_binds,
                vec![menu::Item::Button(fl!("about"), MenuAction::About)],
            ),
        )]);
        vec![menu_bar.into()]
    }

    /// This is the main view of your application, it is the root of your widget tree.
    ///
    /// The `Element` type is used to represent the visual elements of your application,
    /// it has a `Message` associated with it, which dictates what type of message it can send.
    ///
    /// To get a better sense of which widgets are available, check out the `widget` module.
    fn view(&self) -> Element<Self::Message> {

        let shortcut_table = render_shortcuts();

        let whatever =         widget::container(shortcut_table)
            .align_x(Horizontal::Center); // Center the container horizontally

            widget::scrollable(whatever)
                .direction(cosmic::iced_widget::scrollable::Direction::Both {
                    vertical:  cosmic::iced_widget::scrollable::Properties::new(),
                    horizontal:  cosmic::iced_widget::scrollable::Properties::new()
                })
                .width(Length::Fill)
                .into()
    }

    /// Application messages are handled here. The application state can be modified based on
    /// what message was received. Commands may be returned for asynchronous execution on a
    /// background thread managed by the application's executor.
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::LaunchUrl(url) => {
                let _result = open::that_detached(url);
            }
            Message::ToggleContextPage(context_page) => {
                if self.context_page == context_page {
                    self.core.window.show_context = !self.core.window.show_context;
                } else {
                    self.context_page = context_page;
                    self.core.window.show_context = true;
                }
                self.set_context_title(context_page.title());
            }
        }
        Command::none()
    }

    /// Display a context drawer if the context page is requested.
    fn context_drawer(&self) -> Option<Element<Self::Message>> {
        if !self.core.window.show_context {
            return None;
        }
        Some(match self.context_page {
            ContextPage::About => self.about(),
        })
    }
}

impl PopShortcuts {
    /// The about page for this app.
    pub fn about(&self) -> Element<Message> {
        let cosmic_theme::Spacing { space_xxs, .. } = theme::active().cosmic().spacing;
        let icon = widget::svg(widget::svg::Handle::from_memory(
            &include_bytes!("../res/icons/hicolor/128x128/apps/com.github.MyNameIs-13.CosmicPopShortcuts.svg")
                [..],
        ));
        let title = widget::text::title3(fl!("app-title"));
        let link = widget::button::link(REPOSITORY)
            .on_press(Message::LaunchUrl(REPOSITORY.to_string()))
            .padding(0);
        widget::column()
            .push(icon)
            .push(title)
            .push(link)
            .align_items(Alignment::Center)
            .spacing(space_xxs)
            .into()
    }

    /// Updates the header and window titles.
    pub fn update_titles(&mut self) -> Command<Message> {
        let window_title = fl!("app-title");
        let header_title = fl!("app-title");
        self.set_header_title(header_title);
        self.set_window_title(window_title)
    }
}
