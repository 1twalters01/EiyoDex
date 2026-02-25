CREATE TABLE IF NOT EXISTS nutrients_essentiality_types (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

INSERT OR IGNORE INTO mutrients_essentiality_types (id, name) VALUES
(1, 'essential'),
(2, 'conditionally_essential'),
(3, 'non_essential');
