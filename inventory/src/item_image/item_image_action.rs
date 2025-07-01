use crate::error::ShopError;
use crate::item_image::ItemImage;
use crate::{environment, object};
use actix_web::web::{Bytes, Payload};
use futures::StreamExt;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use uuid::Uuid;

impl ItemImage {
    pub fn new(
        item_id: Uuid,
        alt_text: String,
        original_file_name: String,
    ) -> ItemImage {
        ItemImage {
            id: object::random_uuid(),
            item_id,
            alt_text,
            priority: 0, // Priority mechanism is currently unused
            original_file_name,
        }
    }

    pub fn get_item_image_path(&self) -> Result<PathBuf, ShopError> {
        let images_directory = environment::images_directory_path()?;
        Ok(images_directory.join(format!("{}_{}_{}", self.item_id, self.id, self.original_file_name)))
    }

    /// If an error is returned, any created file will be deleted before returning.
    pub async fn store_image_file(&self, payload: &mut Payload) -> Result<(), ShopError> {
        self.store_image_file_impl(payload).await
            .inspect_err(|e| self.store_image_file_error_handler(e))
    }

    async fn store_image_file_impl(&self, payload: &mut Payload) -> Result<(), ShopError> {
        let image_path: PathBuf = self.get_item_image_path()?;
        let mut image_file: File = File::create_new(&image_path)
            .map_err(|e| ShopError::from_error_default(Box::new(e)))?;

        while let Some(chunk) = payload.next().await {
            let chunk: Bytes = chunk
                .map_err(|e| ShopError::from_error_default(Box::new(e)))?;

            image_file.write_all(&chunk)
                .map_err(|e| ShopError::from_error_default(Box::new(e)))?;
        }
        Ok(())
    }

    fn store_image_file_error_handler(&self, e: &ShopError) {
        log::error!("Failed to store image file; Attempting to delete the file; {}", e);
        let path: PathBuf = match self.get_item_image_path() {
            Ok(path) => path,
            Err(e) => {
                log::warn!("File not deleted; {}", e);
                return;
            }
        };
        let remove_result: std::io::Result<()> = fs::remove_file(&path);
        if let Err(e) = remove_result {
            log::warn!("FIle not deleted; {}", e);
        }
    }
}
