-- V3__Marketplace_Integration.sql
-- Add tables for marketplace integration

-- Marketplace listings
CREATE TABLE marketplace_listings (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    inventory_item_id UUID NOT NULL REFERENCES inventory_items(id) ON DELETE RESTRICT,
    marketplace VARCHAR(50) NOT NULL, -- 'ebay', 'etsy', etc.
    external_id VARCHAR(255) NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'active',
    url VARCHAR(1024),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (marketplace, external_id)
);

CREATE INDEX idx_marketplace_listings_inventory_item_id ON marketplace_listings(inventory_item_id);
CREATE INDEX idx_marketplace_listings_status ON marketplace_listings(status);

-- Marketplace sales
CREATE TABLE marketplace_sales (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    listing_id UUID NOT NULL REFERENCES marketplace_listings(id) ON DELETE RESTRICT,
    external_order_id VARCHAR(255) NOT NULL,
    sale_price DECIMAL(10, 2) NOT NULL,
    fees DECIMAL(10, 2) NOT NULL DEFAULT 0,
    sale_date TIMESTAMPTZ NOT NULL,
    buyer_name VARCHAR(255),
    buyer_email VARCHAR(255),
    shipping_address_json JSONB,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_marketplace_sales_listing_id ON marketplace_sales(listing_id);

-- Update triggers
CREATE TRIGGER update_marketplace_listings_updated_at BEFORE UPDATE ON marketplace_listings
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Modify the handle_item_sold function to handle marketplace listings
DROP FUNCTION IF EXISTS handle_item_sold CASCADE;
CREATE OR REPLACE FUNCTION handle_item_sold()
RETURNS TRIGGER AS $$
BEGIN
    -- Update inventory item status
    UPDATE inventory_items 
    SET status = 'sold' 
    WHERE id = NEW.inventory_item_id;
    
    -- Set all related marketplace listings to inactive
    UPDATE marketplace_listings
    SET status = 'inactive'
    WHERE inventory_item_id = NEW.inventory_item_id
    AND status = 'active';
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create trigger for order items (re-create after function update)
CREATE TRIGGER order_item_added AFTER INSERT ON order_items
    FOR EACH ROW EXECUTE FUNCTION handle_item_sold();

-- Create trigger for marketplace sales
CREATE OR REPLACE FUNCTION handle_marketplace_sale()
RETURNS TRIGGER AS $$
DECLARE
    inventory_id UUID;
BEGIN
    -- Get the inventory item ID associated with this listing
    SELECT inventory_item_id INTO inventory_id
    FROM marketplace_listings
    WHERE id = NEW.listing_id;
    
    -- Update inventory item status
    UPDATE inventory_items 
    SET status = 'sold' 
    WHERE id = inventory_id;
    
    -- Set all related marketplace listings to inactive (including the one just sold)
    UPDATE marketplace_listings
    SET status = 'inactive'
    WHERE inventory_item_id = inventory_id
    AND status = 'active';
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER marketplace_sale_recorded AFTER INSERT ON marketplace_sales
    FOR EACH ROW EXECUTE FUNCTION handle_marketplace_sale();
