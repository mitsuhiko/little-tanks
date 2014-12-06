use glfw;
use device;
use render;
use std::error::{Error, FromError};
use std::io;
use serialize::json;
use image;

pub enum GameError {
    EngineInitError,
    WindowInitError,
    InvalidMap(&'static str),

    TextureError(device::tex::TextureError),
    ImageError(image::ImageError),
    ProgramError(render::ProgramError),
    BatchError(render::batch::BatchError),
    IoError(io::IoError),
    JsonParseError(json::ParserError),
    JsonDecodeError(json::DecoderError),
}

fn get_image_error_detail(err: &image::ImageError) -> String {
    use image::ImageError as E;
    match *err {
        E::FormatError(ref x) => x.to_string(),
        E::DimensionError => "wrong dimension".to_string(),
        E::UnsupportedError(ref err) => err.to_string(),
        E::UnsupportedColor(_) => "unsupported color".to_string(),
        E::NotEnoughData => "not enough data".to_string(),
        E::IoError(ref err) => err.description().to_string(),
        E::ImageEnd => "unexpected end of image".to_string(),
    }
}

impl Error for GameError {
    fn description(&self) -> &str {
        match *self {
            GameError::EngineInitError => "failed to initialize engine",
            GameError::WindowInitError => "failed to create window",
            GameError::InvalidMap(desc) => desc,

            GameError::TextureError(_) => "texture error",
            GameError::ImageError(_) => "image error",
            GameError::ProgramError(_) => "program error",
            GameError::BatchError(_) => "batch error",
            GameError::IoError(ref err) => err.description(),
            GameError::JsonParseError(_) => "Could not parse JSON",
            GameError::JsonDecodeError(ref err) => err.description(),
        }
    }

    fn detail(&self) -> Option<String> {
        match *self {
            GameError::TextureError(ref err) => Some(err.to_string()),
            GameError::ImageError(ref err) => Some(get_image_error_detail(err)),
            GameError::ProgramError(ref err) => Some(err.to_string()),
            GameError::BatchError(ref err) => Some(err.to_string()),
            GameError::IoError(ref err) => err.detail(),
            GameError::JsonParseError(_) => None,
            GameError::JsonDecodeError(ref err) => err.detail(),
            _ => None,
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            GameError::IoError(ref err) => Some(err as &Error),
            GameError::JsonDecodeError(ref err) => Some(err as &Error),
            _ => None,
        }
    }
}

impl FromError<device::tex::TextureError> for GameError {
    fn from_error(err: device::tex::TextureError) -> GameError {
        GameError::TextureError(err)
    }
}

impl FromError<render::ProgramError> for GameError {
    fn from_error(err: render::ProgramError) -> GameError {
        GameError::ProgramError(err)
    }
}

impl FromError<render::batch::BatchError> for GameError {
    fn from_error(err: render::batch::BatchError) -> GameError {
        GameError::BatchError(err)
    }
}

impl FromError<io::IoError> for GameError {
    fn from_error(err: io::IoError) -> GameError {
        GameError::IoError(err)
    }
}

impl FromError<json::ParserError> for GameError {
    fn from_error(err: json::ParserError) -> GameError {
        GameError::JsonParseError(err)
    }
}

impl FromError<json::DecoderError> for GameError {
    fn from_error(err: json::DecoderError) -> GameError {
        GameError::JsonDecodeError(err)
    }
}

impl FromError<glfw::InitError> for GameError {
    fn from_error(_: glfw::InitError) -> GameError {
        GameError::EngineInitError
    }
}

impl FromError<image::ImageError> for GameError {
    fn from_error(err: image::ImageError) -> GameError {
        GameError::ImageError(err)
    }
}

pub type Res<T> = Result<T, GameError>;
