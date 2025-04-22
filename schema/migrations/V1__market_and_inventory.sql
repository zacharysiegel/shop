create table if not exists category
(
    id uuid primary key,
    display_name text unique not null,
    internal_name text unique not null,
    parent_id uuid
);
-- This table should never be large enough to warrant an index

create table if not exists product
(
    id uuid primary key,
    display_name text unique not null,
    internal_name text unique not null,
    upc text unique nulls distinct, -- nullable
    release_date date,
    created timestamp with time zone not null,
    updated timestamp with time zone not null
);
create index if not exists idx_product_display_name on product (display_name);
create index if not exists idx_product_upc on product (upc);
create index if not exists idx_product_release_date on product (release_date);

create table if not exists product_category_association
(
    category_id uuid not null references category (id) on delete restrict not null,
    product_id uuid not null references product (id) on delete cascade not null,
    constraint pk_product_category_association primary key (category_id, product_id)
);

create table if not exists inventory_location
(
    id uuid primary key,
    display_name text unique not null,
    internal_name text unique not null
);

create table if not exists item
(
    id uuid primary key,
    product_id uuid references product (id) on delete restrict not null,
    inventory_location_id uuid references inventory_location (id) on delete restrict not null,
    condition int not null, -- corresponds to enum
    status int not null,    -- corresponds to enum
    price_cents bigint not null,
    priority int not null,
    note text,
    acquisition_datetime timestamp with time zone not null,
    acquisition_price_cents bigint,
    acquisition_location text,
    created timestamp with time zone not null,
    updated timestamp with time zone not null
);
create index if not exists idx_item_product_id on item (product_id);
create index if not exists idx_item_price on item (price_cents);

create table if not exists label
(
    id uuid primary key,
    display_name text unique not null,
    internal_name text unique not null
);

create table if not exists item_image
(
    id uuid primary key,
    item_id uuid references item (id) on delete cascade not null,
    uri text not null,
    alt_text text not null,
    priority int not null
);

create table if not exists item_attribute
(
    item_id uuid references item (id) on delete cascade not null,
    key text not null,
    value text not null,
    visible bool not null,
    priority int not null,
    constraint pk_item_attribute primary key (item_id, key)
);

create table if not exists item_label_association
(
    item_id uuid references item (id) on delete cascade not null,
    label_id uuid references label (id) on delete cascade not null,
    constraint pk_item_label_association primary key (item_id, label_id)
);

create table if not exists item_audit
(
    id uuid primary key,
    item_id uuid not null, -- no foreign key constraint in order to retain this field after potential item deletion
    status_before int not null,
    status_after int not null,
    initiated_by_admin bool not null,
    note text,
    created timestamp with time zone not null
);
create index if not exists idx_item_audit_item_id on item_audit (item_id);

create table if not exists metric_counter
(
    id uuid primary key,
    internal_name text unique not null,
    object_id text unique nulls distinct, -- could be a foreign key to several tables, but this is not constrained
    value bigint not null
);

-- On storing addresses: https://web.archive.org/web/20191008203135/http://www.endswithsaurus.com/2009/07/lesson-in-address-storage.html
create table if not exists customer
(
    id uuid primary key,
    email_address text unique not null,
    phone_number text,
    password_hash text not null,
    display_name text not null,
    role int not null,            -- corresponds to enum
    status int not null,          -- corresponds to enum
    shipping_street_address text, -- street number + number suffix + street name + street type + direction + address type + sub id
    shipping_municipality text,   -- minor + major
    shipping_district text,
    shipping_postal_area text,
    shipping_country text,
    billing_street_address text,  -- street number + number suffix + street name + street type + direction + address type + sub id
    billing_municipality text,    -- minor + major
    billing_district text,
    billing_postal_area text,
    billing_country text,
    created timestamp with time zone not null,
    updated timestamp with time zone not null
);

create table if not exists marketplace
(
    id uuid primary key,
    display_name text unique not null,
    internal_name text unique not null,
    uri text unique
);

create table if not exists listing
(
    id uuid primary key,
    item_id uuid references item (id) not null,
    marketplace_id uuid references marketplace (id) not null,
    uri text unique,
    status int not null, -- corresponds to enum
    created timestamp with time zone not null,
    updated timestamp with time zone not null
);
create index if not exists idx_listing_item_id on listing (item_id);
create index if not exists idx_listing_status on listing (status);

create table if not exists purchase
(
    id uuid primary key,
    marketplace_id uuid references marketplace (id) not null,
    external_id text unique nulls distinct,    -- identify an analogous order with an external marketplace
    customer_id uuid references customer (id), -- optional so non-users can still mae
    contact_email_address text not null,
    listing_id uuid references listing (id) not null,
    status int not null,                       -- corresponds to enum
    cost_subtotal_cents bigint not null,
    cost_tax_cents bigint not null,
    cost_shipping_cents bigint not null,
    cont_discount_cents bigint not null,
    seller_cost_total_cents bigint not null,
    shipping_method int not null,              -- corresponds to enum
    payment_method int not null,               -- corresponds to enum
    note text,
    -- Address data is inlined into each purchase record to prevent customer data changes from impacting existing orders
    shipping_street_address text,              -- street number + number suffix + street name + street type + direction + address type + sub id
    shipping_municipality text,                -- minor + major
    shipping_district text,
    shipping_postal_area text,
    shipping_country text,
    billing_street_address text,               -- street number + number suffix + street name + street type + direction + address type + sub id
    billing_municipality text,                 -- minor + major
    billing_district text,
    billing_postal_area text,
    billing_country text,
    created timestamp with time zone not null,
    updated timestamp with time zone not null
);
create index if not exists idx_purchase_external_id on purchase (external_id);
create index if not exists idx_purchase_customer_id on purchase (customer_id);
create index if not exists idx_purchase_status on purchase (status);
create index if not exists idx_purchase_created on purchase (created);
