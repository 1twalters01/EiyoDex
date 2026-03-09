CREATE TABLE IF NOT EXISTS foods_price_metadata (
    id BLOB PRIMARY KEY NOT NULL CHECK(length(id) = 16),
}

