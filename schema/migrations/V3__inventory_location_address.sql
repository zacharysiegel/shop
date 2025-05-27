alter table shop.public.inventory_location
    add column if not exists time_zone_id text not null default 'America/Chicago', -- IANA time zone identifier
    add column if not exists street_address text not null default '1234 Example St.',
    add column if not exists municipality text not null default 'Austin',
    add column if not exists district text not null default 'Texas',
    add column if not exists postal_area text not null default '78726',
    add column if not exists country text not null default 'United States'
;
