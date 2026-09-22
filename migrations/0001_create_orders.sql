CREATE TABLE IF NOT EXISTS orders (
    unid       UUID          PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    -- Flat sku on purpose, no products table/FK: demo app, not real inventory modeling.
    sku        VARCHAR(64)   NOT NULL,
    quantity   INT           NOT NULL CHECK (quantity > 0),
    created_at TIMESTAMPTZ   NOT NULL DEFAULT now()
);
