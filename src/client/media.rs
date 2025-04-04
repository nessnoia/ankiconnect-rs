//! Client for Anki media operations

use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::error::{AnkiError, Result};
use crate::http::{HttpRequestSender, RequestSender};
use crate::models::MediaSource;

use super::request::{self, StoreMediaFileParams};

/// Client for media-related operations
pub struct MediaClient {
    sender: Arc<HttpRequestSender>,
}

impl MediaClient {
    /// Creates a new MediaClient with the given request sender
    pub(crate) fn new(sender: Arc<HttpRequestSender>) -> Self {
        Self { sender }
    }

    /// Stores a media file in Anki's media folder
    ///
    /// # Arguments
    ///
    /// * `source` - The source of the media file
    /// * `filename` - The desired filename in Anki's media folder
    /// * `overwrite` - Whether to overwrite existing files with the same name
    ///
    /// # Returns
    ///
    /// The actual filename that was used (may be different if `overwrite` is false)
    pub fn store_file(
        &self,
        source: &MediaSource,
        filename: &str,
        overwrite: bool,
    ) -> Result<String> {
        if filename.is_empty() {
            return Err(AnkiError::ValidationError(
                "Filename cannot be empty".to_string(),
            ));
        }

        let params = StoreMediaFileParams {
            path: match source {
                MediaSource::Path(path) => Some(path.clone()),
                _ => None,
            },
            url: match source {
                MediaSource::Url(url) => Some(url.clone()),
                _ => None,
            },
            data: match source {
                MediaSource::Base64(data) => Some(data.clone()),
                _ => None,
            },
            filename: filename.to_string(),
            delete_existing: overwrite,
        };

        self.sender.send("storeMediaFile", Some(params))
    }

    /// Stores media from a file path
    ///
    /// Helper method that constructs a MediaSource from a path
    pub fn store_from_path<P: AsRef<Path>>(
        &self,
        path: P,
        filename: &str,
        overwrite: bool,
    ) -> Result<String> {
        let path = path.as_ref();
        if !path.exists() {
            return Err(AnkiError::ValidationError(format!(
                "File does not exist: {}",
                path.display()
            )));
        }

        let source = MediaSource::Path(path.to_path_buf());
        self.store_file(&source, filename, overwrite)
    }

    /// Stores media from a URL
    ///
    /// Helper method that constructs a MediaSource from a URL
    pub fn store_from_url(&self, url: &str, filename: &str, overwrite: bool) -> Result<String> {
        if url.is_empty() {
            return Err(AnkiError::ValidationError(
                "URL cannot be empty".to_string(),
            ));
        }

        let source = MediaSource::Url(url.to_string());
        self.store_file(&source, filename, overwrite)
    }

    /// Stores media from base64 data
    ///
    /// Helper method that constructs a MediaSource from base64 data
    pub fn store_from_base64(&self, data: &str, filename: &str, overwrite: bool) -> Result<String> {
        if data.is_empty() {
            return Err(AnkiError::ValidationError(
                "Base64 data cannot be empty".to_string(),
            ));
        }

        let source = MediaSource::Base64(data.to_string());
        self.store_file(&source, filename, overwrite)
    }

    /// Retrieves a media file from Anki's media folder
    ///
    /// # Arguments
    ///
    /// * `filename` - The name of the file to retrieve
    ///
    /// # Returns
    ///
    /// The file content as base64-encoded data
    pub fn retrieve_file(&self, filename: &str) -> Result<String> {
        if filename.is_empty() {
            return Err(AnkiError::ValidationError(
                "Filename cannot be empty".to_string(),
            ));
        }

        let params = request::RetrieveMediaParams {
            filename: filename.to_string(),
        };

        self.sender.send("retrieveMediaFile", Some(params))
    }

    /// Deletes a media file from Anki's media folder
    ///
    /// # Arguments
    ///
    /// * `filename` - The name of the file to delete
    pub fn delete_file(&self, filename: &str) -> Result<()> {
        if filename.is_empty() {
            return Err(AnkiError::ValidationError(
                "Filename cannot be empty".to_string(),
            ));
        }

        let params = request::DeleteMediaParams {
            filename: filename.to_string(),
        };

        self.sender.send::<_, ()>("deleteMediaFile", Some(params))
    }

    /// Gets the directory where Anki stores media files
    ///
    /// # Returns
    ///
    /// The path to Anki's media folder
    pub fn get_directory(&self) -> Result<PathBuf> {
        let dir: String = self.sender.send("getMediaDirPath", None::<()>)?;
        Ok(PathBuf::from(dir))
    }

    /// Gets a list of missing media files referenced in notes
    ///
    /// # Returns
    ///
    /// A list of missing filenames
    pub fn get_missing_files(&self) -> Result<Vec<String>> {
        self.sender.send("checkMediaDatabase", None::<()>)
    }

    /// Gets the base64-encoded data for an SVG that can be used as a sound icon
    pub fn get_sound_icon(&self) -> Result<String> {
        self.sender.send("getMediaFilesNames", None::<()>)
    }
}
