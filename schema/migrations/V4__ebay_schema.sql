create schema if not exists ebay;

create table if not exists shop.ebay.category
(
    id uuid not null unique primary key,
    ebay_category_id text not null unique,
    ebay_category_tree_id text not null,
    ebay_category_tree_version text not null,
    ebay_category_name text not null
);

insert into shop.ebay.category
(id, ebay_category_id, ebay_category_tree_id, ebay_category_tree_version, ebay_category_name)
values ('00000000-0000-0000-0000-000000000000', '1', '0', '131', 'Collectibles'),
       -- "Star Wars Collectibles" sub-categories
       (gen_random_uuid(), '154', '0', '131', 'Other Star Wars Collectibles'),
       (gen_random_uuid(), '111997', '0', '131', 'Collectible Action Figures'),
       (gen_random_uuid(), '99975', '0', '131', 'Vehicle & Space Ship Replicas')
;

alter table shop.public.category
    add column if not exists ebay_category_id uuid
        not null default '00000000-0000-0000-0000-000000000000'
        references shop.ebay.category
;
