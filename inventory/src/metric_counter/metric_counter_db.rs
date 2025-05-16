use super::*;
use sqlx::postgres::PgQueryResult;
use sqlx::{query, query_as, Error, PgPool};
use uuid::Uuid;

pub async fn get_metric_counter(
    pgpool: &PgPool,
    id: &Uuid,
) -> Result<Option<MetricCounterEntity>, Error> {
    query_as!(
		MetricCounterEntity,
		"\
        select id, internal_name, object_id, value \
        from shop.public.metric_counter \
        where id = $1 \
    ",
		id,
	)
        .fetch_optional(pgpool)
        .await
}

pub async fn create_metric_counter(
    pgpool: &PgPool,
    metric_counter: &MetricCounterEntity,
) -> Result<PgQueryResult, Error> {
    query!(
		"\
        insert into shop.public.metric_counter (id, internal_name, object_id, value)  \
        values ($1, $2, $3, $4) \
    ",
		metric_counter.id,
		metric_counter.internal_name,
		metric_counter.object_id,
		metric_counter.value,
	)
        .execute(pgpool)
        .await
}

pub async fn increment_metric_counter(
    pgpool: &PgPool,
    id: &Uuid,
    increment: &i64,
) -> Result<PgQueryResult, Error> {
    query!(
		"\
        update shop.public.metric_counter \
        set value = value + $1 \
        where id = $2 \
    ",
		increment,
		id,
	)
        .execute(pgpool)
        .await
}
