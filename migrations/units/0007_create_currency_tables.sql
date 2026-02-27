CREATE TABLE IF NOT EXISTS units_currency_types (
    id INTEGER PRIMARY KEY,
    unit_type TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS units_currency_quantities (
    id INTEGER PRIMARY KEY,
    currency_type_id INTEGER NOT NULL,
    quantity REAL NOT NULL,
    FOREIGN KEY (currency_type_id)
        REFERENCES units_currency_types(id)
        ON DELETE CASCADE
);
