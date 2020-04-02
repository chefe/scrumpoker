extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

const APP_TITLE: &str = "Scrum Poker";
const REVEAL_TEXT: &str = "Reveal!";
const ESTIMATION_DEFAULT: &str = "?";
const ESTIMATION_FONT_SIZE: f32 = 200.0;
const BUTTON_FONT_SIZE: f32 = 50.0;

fn main() {
    let app = gtk::Application::new(Some("io.chefe.scrumpoker"), Default::default())
        .expect("Initialization failed...");

    app.connect_activate(move |app| {
        build_ui(app);
    });

    let ret = app.run(&std::env::args().collect::<Vec<_>>());
    std::process::exit(ret);
}

fn build_ui(app: &gtk::Application) {
    let window = create_application_window(app);

    let stack = gtk::Stack::new();
    stack.set_vexpand(true);

    let switcher = gtk::StackSwitcher::new();
    switcher.set_stack(Some(&stack));
    switcher.set_hexpand(true);
    switcher.set_homogeneous(true);
    switcher.set_size_request(-1, 100);
    switcher.set_property_icon_size(gtk::IconSize::Dialog.into());

    let estimation_label = gtk::Label::new(None);
    estimation_label.set_text_with_fontsize(ESTIMATION_DEFAULT, ESTIMATION_FONT_SIZE);

    let reveal_button = create_reveal_button(&stack);

    let grid = gtk::Grid::new();
    grid.set_column_homogeneous(true);
    grid.set_row_homogeneous(true);

    let template = ["1", "2", "3", "5", "8", "13", "?", "∞", "☕"];

    for (i, &item) in template.iter().enumerate() {
        let button = create_button_with_big_font(item);
        grid.attach(&button, (i % 3) as i32, (i / 3) as i32, 1, 1);

        let stack = stack.clone();
        let estimation_label = estimation_label.clone();
        button.connect_clicked(move |_| {
            stack.set_visible_child_name("reveal");
            estimation_label.set_text_with_fontsize(item, ESTIMATION_FONT_SIZE);
        });
    }

    stack.add_named(&grid, "grid");
    stack.set_child_icon_name(&grid, Some("view-app-grid-symbolic"));
    stack.add_named(&reveal_button, "reveal");
    stack.set_child_icon_name(&reveal_button, Some("view-conceal-symbolic"));
    stack.add_named(&estimation_label, "estimation");
    stack.set_child_icon_name(&estimation_label, Some("view-reveal-symbolic"));

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    vbox.add(&stack);
    vbox.add(&switcher);

    window.add(&vbox);
    window.show_all();
}

fn create_application_window(app: &gtk::Application) -> gtk::ApplicationWindow {
    let header_bar = gtk::HeaderBar::new();
    header_bar.set_title(Some(APP_TITLE));
    header_bar.set_show_close_button(true);

    let window = gtk::ApplicationWindow::new(app);
    window.set_title(APP_TITLE);
    window.set_default_size(360, 648);
    window.set_titlebar(Some(&header_bar));

    return window;
}

fn create_reveal_button(stack: &gtk::Stack) -> gtk::Button {
    let stack = stack.clone();

    let button = create_button_with_big_font(REVEAL_TEXT);
    button.connect_clicked(move |_| {
        stack.set_visible_child_name("estimation");
    });

    return button;
}

fn create_button_with_big_font(text: &str) -> gtk::Button {
    let button = gtk::Button::new_with_label("");

    button
        .get_child()
        .unwrap()
        .downcast::<gtk::Label>()
        .unwrap()
        .set_text_with_fontsize(text, BUTTON_FONT_SIZE);

    return button;
}

trait FontSizeHelperForLabel {
    fn set_text_with_fontsize(&self, text: &str, font_size: f32);
}

impl FontSizeHelperForLabel for gtk::Label {
    fn set_text_with_fontsize(&self, text: &str, font_size: f32) {
        self.set_markup(&format!(
            "<span font_desc=\"{}\">{}</span>",
            font_size, text
        ));
    }
}
