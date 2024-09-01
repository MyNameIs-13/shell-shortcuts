use gtk::prelude::*;
use std::rc::Rc;

const CSS: &[u8] = br#"
"#;

const COLUMNS: &[&[Section]] = &[&[
    Section::new(
        "Launcher Shortcuts",
        &[
            Shortcut::new(
                "Activate Launcher",
                Event::Search,
                Schema::Hardcoded(&["Super"]),
            ),
            Shortcut::new(
                "Scroll through the Launcher list",
                Event::ScrollLauncher,
                Schema::Hardcoded(&["↑", "↓"]),
            ),
            Shortcut::new(
                "Execute a command in a terminal",
                Event::ExecuteCommandTerminal,
                Schema::Hardcoded(&["t:"]),
            ),
            Shortcut::new(
                "Execute a command in sh",
                Event::ExecuteCommandSh,
                Schema::Hardcoded(&[":"]),
            ),
            Shortcut::new(
                "Calculate an equation",
                Event::Calculate,
                Schema::Hardcoded(&["="]),
            ),
            Shortcut::new(
                "Show all Applications",
                Event::ShowApplications,
                Schema::Hardcoded(&["Super", "A"]),
            ),
            Shortcut::new(
                "Launch Terminal",
                Event::Terminal,
                Schema::Hardcoded(&["Super", "T"]),
            ),
            Shortcut::new(
                "Launch Browser",
                Event::Browser,
                Schema::Hardcoded(&["Super", "B"]),
            ),
            Shortcut::new(
                "Launch File Manager",
                Event::FileManager,
                Schema::Hardcoded(&["Super", "E"]),
            ),
            Shortcut::new(
                "Launch Text Editor",
                Event::TextEditor,
                Schema::Hardcoded(&["Super", "F"]),
            ),
            Shortcut::new(
                "Launch this overview",
                Event::Overview,
                Schema::Hardcoded(&["Super", "Z"]),
            ),
        ],
    ),
    Section::new(
        "Window & Workspace Management Shortcuts",
        &[
            Shortcut::new(
                "Show Workspace overview",
                Event::ShowWorkspaces,
                Schema::Hardcoded(&["Super", "Tab"]),
            ),
            Shortcut::new(
                "Switch between open windows",
                Event::SwitchWindows,
                Schema::Hardcoded(&["Alt", "Tab"]),
            ),
            Shortcut::new(
                "Switch focus to the workspace above",
                Event::MoveWorkspaceAbove,
                Schema::Hardcoded(&["Ctrl", "Super", "↑"]),
            ),
            Shortcut::new(
                "Switch focus to the workspace below",
                Event::MoveWorkspaceBelow,
                Schema::Hardcoded(&["Ctrl", "Super", "↓"]),
            ),
            Shortcut::new(
                "Switch focus between windows",
                Event::SwitchFocus,
                Schema::Hardcoded(&["Super", "←", "↓", "↑", "→"]),
            ),
            Shortcut::new(
                "Move window",
                Event::MoveWindow,
                Schema::Hardcoded(&["Super", "Shift", "←", "↓", "↑", "→"]),
            ),
            Shortcut::new(
                "Move window up one workspace",
                Event::MoveWorkspaceAbove,
                Schema::Hardcoded(&["Ctrl", "Super", "Shift", "↑"]),
            ),
            Shortcut::new(
                "Move window down one workspace",
                Event::MoveWorkspaceBelow,
                Schema::Hardcoded(&["Ctrl", "Super", "Shift", "↓"]),
            ),
            Shortcut::new(
                "Move window one monitor to the left",
                Event::MoveMonitorLeft,
                Schema::Hardcoded(&["Ctrl", "Super", "Shift", "←"]),
            ),
            Shortcut::new(
                "Move window to one monitor to the right",
                Event::MoveMonitorRight,
                Schema::Hardcoded(&["Ctrl", "Super", "Shift", "→"]),
            ),
            Shortcut::new(
                "Swap windows (keep pressed, then use arrow keys)",
                Event::SwapWindows,
                Schema::Hardcoded(&["Super", "X"]),
            ),
            Shortcut::new(
                "Toggle window orientation",
                Event::OrientationToggle,
                Schema::Hardcoded(&["Super", "O"]),
            ),
            Shortcut::new(
                "Toggle floating mode",
                Event::FloatingToggle,
                Schema::Hardcoded(&["Super", "G"]),
            ),
            Shortcut::new(
                "Toggle auto-tiling",
                Event::AutoTileToggle,
                Schema::Hardcoded(&["Super", "Y"]),
            ),
            Shortcut::new(
                "Toggle stacking mode",
                Event::Stacking,
                Schema::Hardcoded(&["Super", "S"]),
            ),
            Shortcut::new(
                "Toggle maximize",
                Event::MaximizeToggle,
                Schema::Hardcoded(&["Super", "M"]),
            ),
            Shortcut::new(
                "Close window",
                Event::CloseWindow,
                Schema::Hardcoded(&["Super", "Q"]),
            ),
             Shortcut::new(
                "Hide current window",
                Event::HideWindow,
                Schema::Hardcoded(&["Super", "H"]),
            ),
             Shortcut::new(
                "Resize window inward (smaller)",
                Event::ResizeWindowInward,
                Schema::Hardcoded(&["Super", "Shift", "R"]),
            ),
             Shortcut::new(
                "Resize window outward (bigger)",
                Event::ResizeWindowOutward,
                Schema::Hardcoded(&["Super", "R"]),
            ),
        ],
    ),
]];

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Event {
    AutoTileToggle,
    Calculate,
    CloseWindow,
    ExecuteCommandSh,
    ExecuteCommandTerminal,
    FloatingToggle,
    HideWindow,
    MaximizeToggle,
    MoveMonitorLeft,
    MoveMonitorRight,
    MoveWindow,
    MoveWorkspaceAbove,
    MoveWorkspaceBelow,
    Overview,
    OrientationToggle,
    ResizeWindowInward,
    ResizeWindowOutward,
    ScrollLauncher,
    Search,
    ShowApplications,
    Stacking,
    SwapWindows,
    SwitchFocus,
    ShowWorkspaces,
    SwitchWindows,
    Terminal,
    Browser,
    FileManager,
    TextEditor,
}

pub struct Section {
    pub header: &'static str,
    pub shortcuts: &'static [Shortcut],
}

impl Section {
    pub const fn new(header: &'static str, shortcuts: &'static [Shortcut]) -> Self {
        Self { header, shortcuts }
    }
}

pub struct Shortcut {
    pub description: &'static str,
    pub event: Event,
    pub schema: Schema,
}

impl Shortcut {
    pub const fn new(description: &'static str, event: Event, schema: Schema) -> Self {
        Self {
            description,
            event,
            schema,
        }
    }
}

pub enum Schema {
    // GSettings { key: &'static str, from: usize },
    Hardcoded(&'static [&'static str]),
}

pub fn main(app: &gtk::Application) {
    let provider = cascade! {
        gtk::CssProvider::new();
        ..load_from_data(CSS).unwrap();
    };

    gtk::StyleContext::add_provider_for_screen(
        gdk::Screen::default().as_ref().unwrap(),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let shortcuts = cascade! {
        gtk::Box::new(gtk::Orientation::Vertical, 24);
        ..set_border_width(8);
        ..add(&shortcuts_section());
    };

    let scroller = cascade! {
        gtk::ScrolledWindowBuilder::new()
            .hscrollbar_policy(gtk::PolicyType::Never)
            .build();
        ..set_vexpand(true);
        ..add(&shortcuts);
    };

    let content = cascade! {
        gtk::Box::new(gtk::Orientation::Vertical, 24);
        ..add(&scroller);
    };

    let window = cascade! {
        gtk::ApplicationWindow::new(app);
        ..set_size_request(600, 350);
        ..set_default_size(600, 500);
        ..set_icon_name("input-keyboard".into());
        ..add(&content);
        ..show_all();
        ..connect_delete_event(move |window, _| {
            window.close();
            gtk::Inhibit(false)
        });
    };

    window.connect_size_allocate(move |_, allocation| {
        let width = (allocation.width - allocation.width.min(1000)) / 2;

        shortcuts.set_margin_start(width);
        shortcuts.set_margin_end(width);
        shortcuts.set_halign(if width == 0 {
            gtk::Align::Center
        } else {
            gtk::Align::Fill
        });
    });
}

fn shortcuts_section() -> gtk::FlowBox {
    let key_sg = gtk::SizeGroup::new(gtk::SizeGroupMode::Horizontal);

    let container = cascade! {
        gtk::FlowBox::new();
        ..set_selection_mode(gtk::SelectionMode::None);
        ..set_max_children_per_line(2);
        ..set_row_spacing(12);
        ..set_column_spacing(12);
    };

    let event_handler: Rc<dyn Fn(&gtk::EventBox, Event)> = Rc::new(|_widget, event| {
        println!("clicked {:?}", event);
    });

    let iter = COLUMNS.iter().flat_map(|column| {
        column.iter().map(|section| {
            let section = cascade! {
                crate::widgets::Section::new(&key_sg, section, &event_handler);
            };

            section
        })
    });

    for widget in iter {
        container.add(widget.as_ref());
    }

    container
}
