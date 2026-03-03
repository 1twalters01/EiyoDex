CREATE TABLE IF NOT EXISTS journal_timeslot_table (
    id INTEGER PRIMARY key,
    name TEXT NOT NULL,
    description TEXT,
    datetime_created ,
    datetime_last_modified ,
)

CREATE TABLE IF NOT EXISTS journal_timeslot_items (
    id INTEGER PRIMARY KEY,
    journal_timeslot_id INTEGER NOT NULL,
    entry_item_id INTEGER NOT NULL,
    FOREIGN KEY (journal_timeslot_id)
        REFERENCES journal_timeslot_table(id)
        ON DELETE CASCADE,
    FOREIGN KEY (entry_item_id)
        REFERENCES journal_entry_item_table(id)
        ON DELETE CASCADE
)
