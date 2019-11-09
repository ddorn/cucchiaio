

// main.rs

use std::cell::RefCell;
use std::rc::Rc;

use neutrino::{App, Window};
use neutrino::widgets::image::Image;

mod models;
mod listeners;
use models::Images;
use listeners::MyImageListener;
use crate::listeners::{MyPrevButtonListener, MyNextButtonListener, MyWindowListener, QualityLabelListener};
use neutrino::widgets::button::Button;
use neutrino::widgets::container::{Container, Direction};
use neutrino::utils::icon::BreezeIcon;
use neutrino::utils::theme::Theme;
use neutrino::widgets::label::Label;

fn main() {
    let images = Rc::new(RefCell::new(Images::new(".")));
//    let images = Rc::new(RefCell::new(Images::new("/home/diego/rust/cucchiaio/Images_")));

    // Image

    let image_listener = MyImageListener::new(Rc::clone(&images));

    let mut image = Image::from_path("my_image", "");
    image.set_keep_ratio_aspect();
    image.set_listener(Box::new(image_listener));

    // Previous button

    let prev_listener = MyPrevButtonListener::new(Rc::clone(&images));

    let mut button_prev = Button::new("button_prev");
    button_prev.set_icon(Box::new(BreezeIcon::Left));
    button_prev.set_listener(Box::new(prev_listener));

    // Next Button

    let next_listener = MyNextButtonListener::new(Rc::clone(&images));

    let mut button_next = Button::new("button_next");
    button_next.set_icon(Box::new(BreezeIcon::Right));
    button_next.set_listener(Box::new(next_listener));

    // Quality label

    let quality_listener = QualityLabelListener::new(Rc::clone(&images));
    let mut quality_label = Label::new("quality_label");
    quality_label.set_listener(Box::new(quality_listener));

    // Container for the three

    let mut horiz = Container::new("root");
    horiz.set_direction(Direction::Horizontal);
    horiz.add(Box::new(button_prev));
    horiz.add(Box::new(image));
    horiz.add(Box::new(button_next));

    let mut root = Container::new("root");
    root.set_direction(Direction::Vertical);
    root.add(Box::new(quality_label));
    root.add(Box::new(horiz));

    // Handling keypress
    let window_listener = MyWindowListener::new(Rc::clone(&images));

    let mut window = Window::new();
    window.set_title("Image viewer");
    window.set_size(640, 480);
    window.set_child(Box::new(root));
    window.set_listener(Box::new(window_listener));
    window.set_theme(Theme::Adwaita);

    App::run(window);
}