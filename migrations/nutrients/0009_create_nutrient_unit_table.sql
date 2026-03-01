CREATE TABLE IF NOT EXISTS nutrients_unit_types (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);

INSERT OR IGNORE INTO nutrients_unit_types (id, name) VALUES
(1, 'mass'),
(2, 'volume'),
(3, 'energy'),
(4, 'iu'),
(5, 'dfe'),
(6, 'ne'),
(7, 'rae'),
(8, 'pdcaas'),
(9, 'diaas1'),
(10, 'diaas2'),
(11, 'diass3');

CREATE TABLE IF NOT EXISTS nutrients_unit_table (
  id INTEGER PRIMARY KEY,
  unit_type_id INTEGER NOT NULL,
  mass_type_id INTEGER,
  volume_type_id INTEGER,
  energy_type_id INTEGER,
  FOREIGN KEY (unit_type_id)
        REFERENCES nutrients_unit_types(id)
        ON DELETE CASCADE,
  FOREIGN KEY (mass_type_id)
        REFERENCES units_mass_types(id)
        ON DELETE CASCADE,
  FOREIGN KEY (volume_type_id)
        REFERENCES units_volume_types(id)
        ON DELETE CASCADE,
  FOREIGN KEY (energy_type_id)
        REFERENCES units_energy_types(id)
        ON DELETE CASCADE,
  CHECK (
        (mass_type_id IS NOT NULL) +
        (volume_type_id IS NOT NULL) +
        (energy_type_id IS NOT NULL)
        <= 1
  )
)
