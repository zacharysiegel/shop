alter table if exists shop.public.item_image
    drop column if exists uri,
    add column if not exists original_file_name text not null default 'invalid'
;
