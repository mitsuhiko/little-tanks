use std::os;

use image;
use image::DynamicImage;

use map::Map;
use errors::Res;


/// Returns the path to the resource folder.  This tries to be quite
/// resilient but might return the wrong resource folder in case the
/// environment is heavily screwed up.
pub fn get_resource_path() -> Path {
    match os::self_exe_path() {
        Some(exe_path) => {
            exe_path.dir_path().join("resources")
        }
        None => {
            match os::make_absolute(&Path::new("resources")) {
                Ok(rv) => rv,
                Err(_) => Path::new("resources"),
            }
        }
    }
}

pub struct ResourceLoader {
    root: Path,
}

impl ResourceLoader {

    pub fn new() -> ResourceLoader {
        ResourceLoader::new_with_path(get_resource_path())
    }

    pub fn new_with_path(path: Path) -> ResourceLoader {
        ResourceLoader {
            root: path
        }
    }

    pub fn get_filename(&self, category: &str, name: &str) -> Path {
        self.root.join(category).join(name)
    }

    pub fn load_image(&self, name: &str) -> Res<DynamicImage> {
        Ok(try!(image::open(&self.get_filename("images", name))))
    }

    pub fn load_map(&self, name: &str) -> Res<Map> {
        Map::open(&self.get_filename("maps", name))
    }
}
