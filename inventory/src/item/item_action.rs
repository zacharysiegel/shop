use crate::error::ShopError;
use crate::item::Item;
use crate::item_image::{item_image_db, ItemImage};
use crate::ShopEntity;
use sqlx::PgPool;

impl Item {
    pub async fn get_all_item_images(&self, pgpool: &PgPool) -> Result<Vec<ItemImage>, ShopError> {
        item_image_db::get_all_item_images(pgpool, &self.id).await?
            .iter()
            .map(|entity| entity.try_to_model())
            .collect::<Result<Vec<_>, _>>()
    }
}
