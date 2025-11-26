use std::io;
use std::path::Path;

use axum_typed_multipart::FieldData;
use regex::Regex;
use tempfile::NamedTempFile;

use crate::config::app_config::AppConfig;
use crate::errors::app_error::AppError;
use crate::errors::filesystem_error::AppFileSystemError;
use crate::fs::file::SaveFile;
use backoffice_utils::generate::generate_file_name;

#[derive(Debug, Clone)]
pub struct AppFileSystem {
    upload_path: String,
    export_path: String,
}

impl AppFileSystem {
    pub fn new(config: &AppConfig) -> Result<Self, AppError> {
        Self::init(config)?;

        Ok(Self {
            upload_path: config.upload_path.to_owned(),
            export_path: config.export_path.to_owned(),
        })
    }
    pub fn init(config: &AppConfig) -> Result<(), AppError> {
        let fs = Self {
            upload_path: config.upload_path.to_owned(),
            export_path: config.export_path.to_owned(),
        };

        fs.check_or_create_export_path()?;
        fs.check_or_create_upload_path()?;

        tracing::info!("Filesystem initialized");
        Ok(())
    }

    pub fn save_file_to_disk(
        &self,
        document: FieldData<NamedTempFile>,
    ) -> Result<SaveFile, AppFileSystemError> {
        // 1. Prepare the file name
        let original_file_name = &document
            .metadata
            .file_name
            .clone()
            .unwrap_or_else(generate_file_name);

        // Normalize file name: replace whitespace with hyphens
        let sanitized_file_name = Regex::new(r"\s+")
            .unwrap()
            .replace_all(original_file_name, "-")
            .to_string();

        // 2. Save the PDF to disk
        let timestamp = chrono::Local::now().timestamp();
        let temp_dir = Path::new(&self.upload_path);
        let pdf_path = temp_dir.join(format!("{timestamp}_{sanitized_file_name}.jpg")); //TODO: get the extension
        if let Err(err) = document.contents.persist(&pdf_path) {
            log::error!("Failed to persist file: {err}");
            return Err(AppFileSystemError::FailedToSaveToDisk);
        };

        Ok(SaveFile {
            file_name: sanitized_file_name,
            file_path: pdf_path,
        })
    }

    pub fn delete_file_if_exists(&self, path: &str) -> io::Result<()> {
        let file_path = Path::new(path);
        if file_path.exists() {
            std::fs::remove_file(file_path)?;
        }
        Ok(())
    }
}

pub trait FileSystemExt {
    fn check_or_create_export_path(&self) -> Result<(), AppError>;
    fn check_or_create_upload_path(&self) -> Result<(), AppError>;
}

impl FileSystemExt for AppFileSystem {
    fn check_or_create_export_path(&self) -> Result<(), AppError> {
        std::fs::create_dir_all(&self.upload_path).unwrap(); //TODO: improve;

        Ok(())
    }

    fn check_or_create_upload_path(&self) -> Result<(), AppError> {
        std::fs::create_dir_all(&self.export_path).unwrap(); //TODO: Improve
        Ok(())
    }
}
