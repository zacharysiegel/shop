create schema if not exists ebay;

create table shop.ebay.category
(
    id uuid not null unique primary key,
    ebay_category_id text not null unique,
    ebay_category_tree_id text not null,
    ebay_category_tree_version text not null,
    ebay_category_name text
);

insert into shop.ebay.category
(id, ebay_category_id, ebay_category_tree_id, ebay_category_tree_version, ebay_category_name)
values ('00000000-0000-0000-0000-000000000000', '1', '0', '131', 'Collectibles'),
       (gen_random_uuid(), '8675', '0', '131', 'Star Wars Collectibles')
;

alter table shop.public.category
    add column ebay_category_id uuid
        not null default '00000000-0000-0000-0000-000000000000'
        references shop.ebay.category
;
