
use std::cell::RefCell;
use std::rc::Rc;

use neutrino::widgets::image::{ImageListener, ImageState};
use neutrino::utils::pixmap::Pixmap;
use neutrino::utils::event::Key;

use super::models::Images;
use neutrino::widgets::button::{ButtonListener, ButtonState};
use neutrino::WindowListener;
use neutrino::widgets::label::{LabelListener, LabelState};
use crate::models::Quality;

fn quality_to_color(quality: &Quality) -> &str {
    match quality {
        Quality::Good => "#20ff60",
        Quality::Bad => "#ff6030",
        _ => "#fffff",
    }
}

pub struct MyImageListener {
    images: Rc<RefCell<Images>>,
}

impl MyImageListener {
    pub fn new(images: Rc<RefCell<Images>>) -> Self {
        Self { images }
    }
}

impl ImageListener for MyImageListener {
    fn on_update(&self, state: &mut ImageState) {
        let pixmap = Pixmap::from_path(self.images.borrow().selected_path());
        state.set_data(pixmap.data());
        state.set_extension(pixmap.extension());
    }
}

pub struct QualityLabelListener {
    images: Rc<RefCell<Images>>,
}

impl QualityLabelListener {
    pub fn new(images: Rc<RefCell<Images>>) -> Self {
        Self { images }
    }
}

impl LabelListener for QualityLabelListener {
    fn on_update(&self, state: &mut LabelState) {
        let label = match self.images.borrow().selected_quality() {
            Quality::Good => "Good",
            Quality::Bad => "Bad",
            Quality::Neutral => "Neutral",
            Quality::NotSeen => "Not Seen",
        };
        state.set_text(label);
        state.set_style(&format!("background: {}; text-align: center", quality_to_color(self.images.borrow().selected_quality())));
    }
}

pub struct MyPrevButtonListener {
    images: Rc<RefCell<Images>>,
}

impl MyPrevButtonListener {
    pub fn new(images: Rc<RefCell<Images>>) -> Self {
        Self { images }
    }
}

impl ButtonListener for MyPrevButtonListener {
    fn on_update(&self, _state: &mut ButtonState) {
        _state.set_style(&format!("background: {}", quality_to_color(self.images.borrow().previous_quality())));
    }

    fn on_change(&self, _state: &ButtonState) {
        self.images.borrow_mut().previous();
    }
}

pub struct MyNextButtonListener {
    images: Rc<RefCell<Images>>,
}

impl MyNextButtonListener {
    pub fn new(images: Rc<RefCell<Images>>) -> Self {
        Self { images }
    }
}

impl ButtonListener for MyNextButtonListener {
    fn on_update(&self, _state: &mut ButtonState) {

        _state.set_style(&format!("background: {}", quality_to_color(self.images.borrow().next_quality())));
    }

    fn on_change(&self, _state: &ButtonState) {
        self.images.borrow_mut().next();
    }
}


pub struct MyWindowListener {
    images: Rc<RefCell<Images>>,
}

impl MyWindowListener {
    pub fn new(images: Rc<RefCell<Images>>) -> Self {
        Self { images }
    }
}

impl WindowListener for MyWindowListener {
    fn on_key(&self, key: Key) {
        let mut images = self.images.borrow_mut();
        match key {
            Key::H => images.previous(),
            Key::L => {
                if let Quality::NotSeen = images.selected_quality() {
                    images.set_quality(Quality::Neutral);
                }
                images.next();
            },
            Key::X | Key::J => {
                images.set_quality(Quality::Bad);
                images.next()
            }
            Key::K => {
                images.set_quality(Quality::Good);
                images.next()
            }
            Key::R => {
                images.set_quality(Quality::Neutral);
                images.next()
            }
            Key::P => {
                images.process();
            }
            _ => (),
        }
    }

    fn on_tick(&self) {
        unimplemented!()
    }
}