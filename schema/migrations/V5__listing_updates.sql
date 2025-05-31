alter table if exists shop.public.listing
    drop column if exists uri
;

alter table if exists shop.public.listing
    add constraint item_marketplace_unique unique (item_id, marketplace_id)
;
