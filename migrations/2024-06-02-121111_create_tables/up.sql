-- Your SQL goes here

-- Create statuses table
CREATE TABLE statuses (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    color TEXT NOT NULL
);

-- Create tags table
CREATE TABLE tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    color TEXT NOT NULL
);

-- Create notebooks table
CREATE TABLE notebooks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    status_id INTEGER,
    notebook_id INTEGER,
    FOREIGN KEY (status_id) REFERENCES statuses(id),
    FOREIGN KEY (notebook_id) REFERENCES notebooks(id)
);

-- Create notes table
CREATE TABLE notes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    text TEXT NOT NULL,
    status_id INTEGER,
    notebook_id INTEGER,
    FOREIGN KEY (status_id) REFERENCES statuses(id),
    FOREIGN KEY (notebook_id) REFERENCES notebooks(id)
);

-- Create notebook_tags table
CREATE TABLE notebook_tags (
    notebook_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    FOREIGN KEY (notebook_id) REFERENCES notebooks(id),
    FOREIGN KEY (tag_id) REFERENCES tags(id),
    PRIMARY KEY (notebook_id, tag_id)
);

-- Create note_tags table
CREATE TABLE note_tags (
    note_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    FOREIGN KEY (note_id) REFERENCES notes(id),
    FOREIGN KEY (tag_id) REFERENCES tags(id),
    PRIMARY KEY (note_id, tag_id)
);