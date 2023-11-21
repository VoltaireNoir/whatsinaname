use parse_display::Display;

pub const INVALID_CHARS: &[char] = &[
    '!', '@', '#', '$', '%', '^', '&', '*', '{', '}', '/', '\\', ',', '<', '>', '?', ':', ';',
    '\'', '|', '=', '+', '`',
];

#[cfg(feature = "image")]
pub const IMAGE_FORMATS: &[&str] = &[
    "jpg", "jpeg", "jpe", "jfif", "jif", "png", "gif", "bmp", "svg", "svgz", "raw", "arw", "cr2",
    "nrw", "k25", "webp", "tiff", "tif", "heif", "helc", "jp2", "j2k", "jpf", "jpx", "jpm", "mj2",
    "eps",
];

#[cfg(feature = "proprietary")]
pub const PROPRIETARY_FORMATS: &[&str] = &["psd", "ind", "indt", "indd", "ai"];

pub const EXECUTABLE_FORMATS: &[&str] = &["action", "exe", "bat"];

pub trait AboutFile {
    fn has_invalid_chars(&self, custom_chars: Option<&[char]>) -> bool;
    fn has_extension(&self, extensions: &[&str]) -> bool;
    fn has_some_extension(&self) -> bool;
    fn has_replacement_char(&self) -> bool;
    fn get_extension(&self) -> Option<&str>;
    fn get_name(&self) -> &str;
    fn is_valid_filename(&self) -> bool;
    fn is_valid_file_with_ext(&self, extensions: &[&str]) -> bool;

    #[cfg(feature = "executable")]
    fn is_executable(&self) -> bool;

    #[cfg(feature = "executable")]
    fn is_valid_executable(&self) -> bool;

    #[cfg(feature = "image")]
    fn is_image(&self) -> bool;

    #[cfg(feature = "image")]
    fn is_valid_image(&self) -> bool;

    #[cfg(feature = "file-type")]
    fn file_type(&self) -> FileType;
}

#[cfg(feature = "file-type")]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FileType {
    Unknown,
    Image(ImgType),
    Executable(ExecType),
    Proprietary(PropType),
    Document(DocType),
    Archive(ArchType),
    Audio,
}

#[cfg(feature = "file-type")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Display)]
pub enum ExecType {
    #[display("EXE - Windows Executable")]
    EXE,
    #[display("ACTION - MacOs Automator Action")]
    ACTION,
    #[display("BAT - Windows Batch File")]
    BAT,
}

impl ExecType {
    fn get(ext: &str) -> Self {
        match ext {
            "exe" => ExecType::EXE,
            "action" => ExecType::ACTION,
            "bat" => ExecType::BAT,
            _ => unreachable!(),
        }
    }
}

#[cfg(feature = "file-type")]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PropType {
    PSD,
    AI,
    INDD, // INDD (.ind, .indd, .indt)
}

impl PropType {
    fn get(ext: &str) -> Self {
        match ext {
            "psd" => PropType::PSD,
            "ind" | "indd" | "indt" => PropType::INDD,
            "ai" => PropType::AI,
            _ => unreachable!(),
        }
    }
}

#[cfg(feature = "file-type")]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DocType {}

#[cfg(feature = "file-type")]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ArchType {}

#[cfg(feature = "file-type")]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ImgType {
    JPEG,
    PNG,
    GIF,
    BMP,
    SVG,
    RAW,
    WEBP,
    TIFF,
    PSD,
    HEIF,
    JPEG2000,
    EPS,
}

#[cfg(feature = "file-type")]
impl ImgType {
    fn get(ext: &str) -> ImgType {
        match ext {
            "jpg" | "jpeg" | "jpe" | "jfif" | "jif" => ImgType::JPEG,
            "png" => ImgType::PNG,
            "gif" => ImgType::GIF,
            "bmp" => ImgType::BMP,
            "svg" | "svgz" => ImgType::SVG,
            "raw" | "arw" | "cr2" | "nrw" | "k25" => ImgType::RAW,
            "webp" => ImgType::WEBP,
            "tiff" | "tif" => ImgType::TIFF,
            "psd" => ImgType::PSD,
            "heif" | "helc" => ImgType::HEIF,
            "jp2" | "j2k" | "jpf" | "jpx" | "jpm" | "mj2" => ImgType::JPEG2000,
            "eps" => ImgType::EPS,
            _ => unreachable!(),
        }
    }
}

impl AboutFile for str {
    fn has_invalid_chars(&self, custom_chars: Option<&[char]>) -> bool {
        let invalid = custom_chars.unwrap_or(INVALID_CHARS);
        invalid.iter().map(|c| self.contains(*c)).any(|b| b)
    }

    fn has_extension(&self, extensions: &[&str]) -> bool {
        if let Some(ext) = self.get_extension() {
            extensions.iter().any(|e| e == &ext.to_lowercase())
        } else {
            false
        }
    }

    fn has_some_extension(&self) -> bool {
        if let Some(i) = self.rfind('.') {
            !self[i..].has_invalid_chars(None)
        } else {
            false
        }
    }

    fn has_replacement_char(&self) -> bool {
        self.contains(std::char::REPLACEMENT_CHARACTER)
    }

    fn get_extension(&self) -> Option<&str> {
        if self.has_some_extension() {
            let (_, ext) = self.split_at(self.rfind('.').unwrap());
            Some(ext[1..].trim())
        } else {
            None
        }
    }

    fn get_name(&self) -> &str {
        if self.has_some_extension() {
            let (name, _) = self.split_at(self.rfind('.').unwrap());
            name
        } else {
            self
        }
    }

    fn is_valid_filename(&self) -> bool {
        !self.has_invalid_chars(None) & !self.starts_with(' ') & !self.has_replacement_char()
    }

    fn is_valid_file_with_ext(&self, extensions: &[&str]) -> bool {
        if self.is_valid_filename() && self.has_some_extension() {
            let ext = self.get_extension().unwrap();
            extensions.iter().any(|e| *e == ext.to_lowercase())
        } else {
            false
        }
    }

    #[cfg(feature = "executable")]
    fn is_executable(&self) -> bool {
        if let Some(ext) = self.get_extension() {
            EXECUTABLE_FORMATS.contains(&ext)
        } else {
            false
        }
    }

    #[cfg(feature = "executable")]
    fn is_valid_executable(&self) -> bool {
        self.is_valid_filename() && self.is_executable()
    }

    #[cfg(feature = "image")]
    fn is_image(&self) -> bool {
        IMAGE_FORMATS
            .iter()
            .map(|f| self.to_lowercase().ends_with(f))
            .any(|b| b)
    }

    #[cfg(feature = "image")]
    fn is_valid_image(&self) -> bool {
        self.is_image() && self.is_valid_filename()
    }

    #[cfg(feature = "file-type")]
    fn file_type(&self) -> FileType {
        if let Some(ext) = self.get_extension() {
            if self.is_image() {
                FileType::Image(ImgType::get(ext))
            } else if self.is_executable() {
                FileType::Executable(ExecType::get(ext))
            } else {
                FileType::Unknown
            }
        } else {
            FileType::Unknown
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "image")]
    fn valid_image() {
        let name = "should_be_valid.jpg";
        assert!(name.is_valid_image())
    }

    #[test]
    #[cfg(feature = "image")]
    fn invalid_image() {
        let name = "  not$allowed";
        assert!(!name.is_valid_image())
    }

    #[test]
    fn extension() {
        assert_eq!("test.tst".get_extension().unwrap(), "tst")
    }

    #[test]
    fn custom_extensions() {
        let fname = "myfile.custom";
        assert!(fname.is_valid_file_with_ext(&["custom", "cust"]))
    }
}
