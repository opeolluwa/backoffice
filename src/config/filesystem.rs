use crate::{ errors::app_error::AppError};
use crate::config::app_config::AppConfig;

pub struct AppFileSystem {
    upload_path: String,
    export_path: String,
}

impl AppFileSystem {
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
}

pub trait FileSystemExt {
    fn check_or_create_export_path(&self) -> Result<(), AppError>;
    fn check_or_create_upload_path(&self) -> Result<(), AppError>;
}

impl FileSystemExt for AppFileSystem {
    fn check_or_create_export_path(&self) -> Result<(), AppError> {
        std::fs::create_dir_all(&self.upload_path).unwrap();//TODO: improve;

        Ok(())
    }

    fn check_or_create_upload_path(&self) -> Result<(), AppError> {
        std::fs::create_dir_all(&self.export_path).unwrap();//TODO: Improve
        Ok(())
    }
}