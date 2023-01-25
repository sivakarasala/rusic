use std::env;

use gio::{ApplicationExt, ApplicationExtManual, ApplicationFlags};
use gtk::{Application, ApplicationWindow, GtkWindowExt, WidgetExt};
use gtk::{ContainerExt, SeparatorToolItem, ToolButton, Toolbar};

const PLAY_STOCK: &str = "gtk-media-play";

fn main() {
    let application = Application::new("com.github.rusic", ApplicationFlags::empty())
        .expect("Application initialization failed");
    application.connect_startup(|application| {
        let window = ApplicationWindow::new(&application);
        window.set_title("Rusic");
        let toolbar = Toolbar::new();
        window.add(&toolbar);
        let open_button = ToolButton::new_from_stock("gtk-open");
        toolbar.add(&open_button);
        toolbar.add(&SeparatorToolItem::new());

        let previous_button = ToolButton::new_from_stock("gtk-media-previous");
        toolbar.add(&previous_button);

        let play_button = ToolButton::new_from_stock(PLAY_STOCK);
        toolbar.add(&play_button);

        let stop_button = ToolButton::new_from_stock("gtk-media-stop");
        toolbar.add(&stop_button);

        let next_button = ToolButton::new_from_stock("gtk-media-next");
        toolbar.add(&next_button);

        toolbar.add(&SeparatorToolItem::new());

        let remove_button = ToolButton::new_from_stock("gtk-remove");
        toolbar.add(&remove_button);

        toolbar.add(&SeparatorToolItem::new());

        let quit_button = ToolButton::new_from_stock("gtk-quit");
        toolbar.add(&quit_button);
        window.show_all();
    });
    application.connect_activate(|_| {});
    application.run(&env::args().collect::<Vec<_>>());
}
