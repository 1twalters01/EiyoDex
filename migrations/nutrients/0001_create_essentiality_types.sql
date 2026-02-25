CREATE TABLE essentiality_types (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

INSERT INTO essentiality_types (id, name) VALUES
(1, 'essential'),
(2, 'conditionally_essential'),
(3, 'non_essential');
