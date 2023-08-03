use std::path::PathBuf;

use mime_guess::{Mime, MimeGuess};

pub trait PathBufDetemineMimeExt {
    fn get_mime_type(&self) -> Mime;
}

impl PathBufDetemineMimeExt for PathBuf {
    fn get_mime_type(&self) -> Mime {
        MimeGuess::from_ext(self.get_file_ext()).first_or_text_plain()
    }
}

pub trait PathBufFileUtilExt {
    fn is_file(&self) -> bool;
    fn get_file_ext(&self) -> &str;
    fn get_file_name(&self) -> &str;
}

impl PathBufFileUtilExt for PathBuf {
    fn is_file(&self) -> bool {
        let parts = &self.to_str();

        if parts.is_none() {
            return false;
        }

        let mut parts = parts.unwrap().split('.');

        parts.next().is_some() && parts.next().is_some()
    }

    fn get_file_ext(&self) -> &str {
        let parts = self.to_str().unwrap().split('.');

        parts.last().unwrap()
    }

    fn get_file_name(&self) -> &str {
        let parts = self.to_str().unwrap().split('.');

        parts.last().unwrap()
    }
}
