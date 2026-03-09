CREATE TABLE IF NOT EXISTS journal_journal_entry_table (
    id INTEGER PRIMARY KEY,
    profile_id INTEGER NOT NULL,
    entry_note TEXT,
    date Integer,
);

CREATE TABLE IF NOT EXISTS journal_uncategorised_time_slots (
    id INTEGER PRIMARY KEY,
    journal_entry_id INTEGER,
    time_slot_id INTEGER
);
