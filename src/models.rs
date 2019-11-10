use crate::models::Quality::{Neutral, NotSeen};
use std::path::{PathBuf};

pub enum Quality {
    Bad,
    Good,
    Neutral,
    NotSeen,
}

pub struct Image {
    path: PathBuf,
    quality: Quality,
}

impl Image {
    pub fn new(path: &str) -> Self {
        Self {
            path: PathBuf::from(path),
            quality: NotSeen,
        }
    }
}

pub struct Images {
    directory: PathBuf,
    paths: Vec<Image>,
    selected: usize,
}

impl Images {
    pub fn new(dir_path: &str) -> Self {
        let mut paths : Vec<String> = std::fs::read_dir(dir_path).unwrap()
            .filter_map(|p| p.ok())
            .map(|p| p.path().into_os_string().into_string().unwrap())
            .filter(|p| p.ends_with(".jpg"))
            .collect();
        paths.sort();

        Self {
            directory: std::path::PathBuf::from(dir_path),
            paths: paths.iter().map(|p| Image::new(&p)).collect(),
            selected: 0,
        }
    }

    pub fn selected_path(&self) -> &str {
        let path = &self.paths[self.selected].path;
        println!("{}", path.to_str().unwrap());
        path.to_str().unwrap()
    }

    pub fn selected_quality(&self) -> &Quality { &self.paths[self.selected].quality}
    pub fn previous_quality(&self) -> &Quality { &self.paths[(self.selected + self.paths.len() - 1) % self.paths.len()].quality}
    pub fn next_quality(&self) -> &Quality { &self.paths[(self.selected + 1) % self.paths.len()].quality}

    pub fn set_quality(&mut self, quality: Quality) {
        self.paths[self.selected].quality = quality;
    }

    pub fn next(&mut self) {
        self.selected = if self.selected == self.paths.len() - 1 {
            0
        } else {
            self.selected + 1
        }
    }

    pub fn previous(&mut self) {
        self.selected = if self.selected == 0 {
            self.paths.len() - 1
        } else {
            self.selected - 1
        }
    }

    pub fn process(&self) {
        let bad = self.directory.join("bad");
        let good = self.directory.join("good");
        let ok = self.directory.join("ok");

        std::fs::create_dir(&bad);
        std::fs::create_dir(&good);
        std::fs::create_dir(&ok);

        for img in &self.paths {
            let filename = img.path.file_name().unwrap();
            let folder = match img.quality {
                Quality::Bad => &bad,
                Quality::Good => &good,
                Neutral => &ok,
                NotSeen => continue,
            };
            std::fs::rename(&img.path, folder.join(filename));
        }


    }
}