use super::*;
use sqlx::postgres::PgQueryResult;
use sqlx::{query, query_as, PgPool};
use uuid::Uuid;

pub async fn get_label(pgpool: &PgPool, label_id: Uuid) -> Result<Option<Label>, sqlx::Error> {
    query_as!(Label, "\
        select * \
        from label \
        where id = $1 \
    ", label_id)
        .fetch_optional(pgpool)
        .await
}

pub async fn create_label(pgpool: &PgPool, label: &Label) -> Result<PgQueryResult, sqlx::Error> {
    query!("\
        insert into label (id, display_name, internal_name) \
        values ($1, $2, $3) \
    ",
        label.id,
        label.display_name,
        label.internal_name,
    )
        .execute(pgpool)
        .await
}

pub async fn get_all_labels(pgpool: &PgPool) -> Result<Vec<Label>, sqlx::Error> {
    query_as!(Label, "\
        select id, display_name, internal_name \
        from label \
    ")
        .fetch_all(pgpool)
        .await
}
