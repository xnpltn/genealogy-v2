#![allow(dead_code)]

pub fn create_tables() -> String {
    let query = r#"

-- create table for relative
CREATE TABLE IF NOT EXISTS relative (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    created_at      DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at      DATETIME DEFAULT CURRENT_TIMESTAMP,
    sameness        FLOAT DEFAULT 0.0,
    lost_reason     TEXT,
    atnum           INTEGER,
    sex             TEXT NOT NULL,
    birthday        DATETIME,  -- Corrected from "bithday"
    died_at         DATETIME,
    age             INTEGER,
    fname           TEXT NOT NULL,
    mname           TEXT,
    lname           TEXT NOT NULL, 
    full_name       TEXT,
    phone           TEXT,
    email           TEXT,
    mother_id       INTEGER,
    father_id       INTEGER,
    end_of_line     BOOLEAN DEFAULT 1,
    pinned          BOOLEAN DEFAULT 0,
    hotness         INTEGER DEFAULT 0,
    crazy           INTEGER DEFAULT 0,
    swarthy         INTEGER DEFAULT 0,
    employable      INTEGER DEFAULT 0,
    default_image_id INTEGER,
    address         TEXT,
    state           TEXT,
    city            TEXT,
    zipcode         TEXT,
    FOREIGN KEY (mother_id) REFERENCES relative(id),
    FOREIGN KEY (father_id) REFERENCES relative(id)
);

-- Trigger to update the updated_at field on update on relative
CREATE TRIGGER IF NOT EXISTS update_relative_updated_at
AFTER UPDATE ON relative
FOR EACH ROW
BEGIN
    UPDATE relative SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.id;
END;


-- Trigger to set full_name to fname + lname on INSERT or UPDATE
CREATE TRIGGER IF NOT EXISTS set_full_name_insert
AFTER INSERT ON relative
FOR EACH ROW
BEGIN
    UPDATE relative
    SET full_name = NEW.fname || ' ' || NEW.lname
    WHERE id = NEW.id;
END;


CREATE TRIGGER IF NOT EXISTS set_full_name_update
AFTER UPDATE ON relative
FOR EACH ROW
BEGIN
    UPDATE relative
    SET full_name = NEW.fname || ' ' || NEW.lname
    WHERE id = NEW.id;
END;


-- Calculate age after creating an individual
CREATE TRIGGER IF NOT EXISTS calculate_age_after_insert
AFTER INSERT ON relative
BEGIN
    UPDATE relative
    SET age = CAST((julianday('now') - julianday(birthday)) / 365.25 AS INTEGER)
    WHERE id = NEW.id;
END;

-- Calculate age after updating birthday
CREATE TRIGGER IF NOT EXISTS calculate_age_after_update
AFTER UPDATE OF birthday ON relative
WHEN NEW.birthday != OLD.birthday
BEGIN
    UPDATE relative
    SET age = CAST((julianday('now') - julianday(NEW.birthday)) / 365.25 AS INTEGER)
    WHERE id = NEW.id;
END;

-- Table file to store files related to relative
CREATE TABLE IF NOT EXISTS file (
    id                  INTEGER PRIMARY KEY AUTOINCREMENT,
    created_at          DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at          DATETIME DEFAULT CURRENT_TIMESTAMP,
    file_name           TEXT NOT NULL,
    file_path           TEXT NOT NULL,
    relative_id         INTEGER  NOT NULL,
    type                TEXT NOT NULL,
    size                TEXT NOT NULL,
    pinned              BOOLEAN DEFAULT 0,
    FOREIGN KEY         (relative_id) REFERENCES relative(id) ON DELETE CASCADE
);

-- Trigger to update filename_timestamp on file update
CREATE TRIGGER IF NOT EXISTS update_files_updated_at
AFTER UPDATE ON file
FOR EACH ROW
BEGIN 
  UPDATE file SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.id;
END;

-- Table to store notes related to relative
CREATE TABLE IF NOT EXISTS note (
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    created_at       DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at       DATETIME DEFAULT CURRENT_TIMESTAMP,
    relative_id      INTEGER NOT NULL,
    text             TEXT NOT NULL,
    pinned           BOOLEAN DEFAULT 0,
    FOREIGN KEY      (relative_id) REFERENCES relative(id) ON DELETE CASCADE
);

-- Trigger for updating updated_at on notes
CREATE TRIGGER IF NOT EXISTS update_notes_updated_at
AFTER UPDATE ON note
FOR EACH ROW
BEGIN
    UPDATE note SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.id;
END;



CREATE TABLE IF NOT EXISTS image (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    created_at      DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at      DATETIME DEFAULT CURRENT_TIMESTAMP,
    relative_id     INTEGER NOT NULL,
    filename        TEXT NOT NULL,
    pinned          BOOLEAN DEFAULT 0,
    FOREIGN KEY     (relative_id) REFERENCES relative(id) ON DELETE CASCADE
);



-- Trigger for updating updated_at on notes
CREATE TRIGGER IF NOT EXISTS update_image_updated_at
AFTER UPDATE ON image
FOR EACH ROW
BEGIN
    UPDATE image SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.id;
END;

    "#;
    query.to_string()
}

pub fn create_new_relative_no_parents() -> String {
    let query = r#"
        INSERT INTO relative (
            sameness, 
            lost_reason, 
            sex, 
            birthday, 
            fname, 
            mname, 
            lname, 
            phone, 
            email, 
            pinned, 
            employable, 
            swarthy, 
            hotness, 
            crazy, 
            died_at,
            address,
            state
        ) VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, 
            $10, $11, $12, $13, $14, $15, $16, 
            $17
        ) RETURNING id
    "#;

    query.to_string()
}

pub fn create_new_relative_with_mother_only() -> String {
    let query = r#"
        INSERT INTO relative (
            sameness, 
            lost_reason, 
            sex, 
            birthday, 
            fname, 
            mname, 
            lname, 
            phone, 
            email, 
            pinned, 
            mother_id, 
            employable, 
            swarthy, 
            hotness, 
            crazy, 
            died_at,
            address,
            state
        ) VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, 
            $10, $11, $12, $13, $14, $15, $16, 
            $17, $18
        ) RETURNING id
    "#;
    query.to_string()
}
pub fn create_new_relative_with_father_only() -> String {
    let query = r#"
        INSERT INTO relative (
            sameness, 
            lost_reason, 
            sex, 
            birthday, 
            fname, 
            mname, 
            lname, 
            phone, 
            email, 
            pinned, 
            father_id, 
            employable, 
            swarthy, 
            hotness, 
            crazy, 
            died_at,
            address,
            state
        ) VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, 
            $10, $11, $12, $13, $14, $15, $16, 
            $17, $18
        ) RETURNING id
    "#;
    query.to_string()
}

pub fn create_new_relative_with_both_parents() -> String {
    let query = r#"
        INSERT INTO relative (
            sameness, 
            lost_reason, 
            sex, 
            birthday, 
            fname, 
            mname, 
            lname, 
            phone, 
            email, 
            pinned, 
            mother_id, 
            father_id, 
            employable, 
            swarthy, 
            hotness, 
            crazy, 
            died_at,
            address,
            state
        ) VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, 
            $10, $11, $12, $13, $14, $15, $16, 
            $17, $18, $19
        ) RETURNING id
    "#;
    query.to_string()
}

pub fn get_all_relatives() -> String {
    let query = r#"
        SELECT 
            *
        FROM 
            relative
        ORDER BY
            pinned
        DESC
        ;
    "#;

    query.to_string()
}

pub fn get_female_relatives() -> String {
    let query = r#"
        SELECT 
            *
        FROM 
            relative
        WHERE
            LOWER(sex) = LOWER('female') AND hotness > 0
        ORDER BY
            pinned DESC
        ;
    "#;

    query.to_string()
}

pub fn get_all_employees() -> String {
    let query = r#"
        SELECT 
            *
        FROM 
            relative 
        WHERE 
            LOWER(sex) = LOWER('male') AND employable > 0
        ORDER BY 
            pinned
        DESC
        ;
    "#;
    query.to_string()
}

pub fn get_one_relative_data() -> String {
    let query = r#"
        SELECT
            *
        FROM
            relative
        WHERE 
            id = $1
        ;
    "#;
    query.to_string()
}
pub fn update_mother_only() -> String {
    let query = r#"
            UPDATE relative 
            SET 
                sameness = $1,
                lost_reason = $2,
                sex = $3,
                birthday = $4,
                fname = $5,
                mname = $6,
                lname = $7,
                phone = $8,
                email = $9,
                pinned = $10,
                mother_id =  $11,
                employable = $12 ,
                swarthy = $13, 
                hotness = $14, 
                crazy = $15,
                died_at= $16,
                address = $17,
                state = $18

            WHERE 
                id = $19
            ;

    "#
    .to_string();
    query
}

pub fn update_father_only() -> String {
    let query = r#"
            UPDATE relative 
            SET 
                sameness = $1,
                lost_reason = $2,
                sex = $3,
                birthday = $4,
                fname = $5,
                mname = $6,
                lname = $7,
                phone = $8,
                email = $9,
                pinned = $10,
                father_id = $11,
                employable = $12,
                swarthy = $13, 
                hotness = $14, 
                crazy = $15,
                died_at = $16,
                address = $17,
                state = $18
            WHERE 
                id = $19
            ;
    "#
    .to_string();
    query
}

pub fn update_both_parents() -> String {
    let query = r#"
            UPDATE relative 
            SET 
                sameness = $1,
                lost_reason = $2,
                sex = $3,
                birthday = $4,
                fname = $5,
                mname = $6,
                lname = $7,
                phone = $8,
                email = $9,
                pinned = $10,
                father_id = $11,
                mother_id = $12,
                employable = $13 ,
                swarthy = $14, 
                hotness = $15, 
                crazy = $16,
                died_at = $17,
                address = $18,
                state = $19
            WHERE 
                id = $20
            ;
    "#
    .to_string();
    query
}

pub fn update_no_parents() -> String {
    let query = r#"
            UPDATE relative 
            SET 
                sameness = $1,
                lost_reason = $2,
                sex = $3,
                birthday = $4,
                fname = $5,
                mname = $6,
                lname = $7,
                phone = $8,
                email = $9,
                pinned = $10,
                employable = $11 ,
                swarthy = $12, 
                hotness = $13, 
                crazy = $14,
                died_at = $15,
                address = $16,
                state = $17
            WHERE 
                id = $18
            ;

    "#
    .to_string();
    query
}
pub fn get_females() -> String {
    let query = r#"
        SELECT 
            full_name
        FROM 
            relative
        WHERE
            LOWER(sex) = LOWER('female')
        ORDER BY
            pinned
        DESC
        ;
    "#;
    query.to_string()
}

pub fn get_males() -> String {
    let query = r#"
        SELECT 
            full_name
        FROM 
            relative
        WHERE
            LOWER(sex) = LOWER('male')
            AND age > 13
        ORDER BY
            pinned
        DESC
        ;
    "#;
    query.to_string()
}

pub fn add_file() -> String {
    let query = r#"
        INSERT INTO file (
            imported_name, 
            imported_hash, 
            relative_id, 
            type, 
            size, 
            filename, 
            filename_hashname, 
            file_directory
        ) VALUES (
            $1, 'xzy', $2, $3, $4, $5, 'xzy', 'files'
        );
    "#;

    query.to_string()
}

pub fn get_files_for_relative() -> String {
    let query = r#"
        SELECT * FROM file WHERE relative_id = (
            SELECT id FROM relative WHERE id = $1
        ) ORDER BY pinned DESC;
    "#;
    query.to_string()
}

pub fn get_notes_for_relative() -> String {
    let query = r#"
        SELECT * FROM note WHERE relative_id = (
            SELECT id FROM relative WHERE id = $1
        ) ORDER BY pinned DESC;
    "#;
    query.to_string()
}

pub fn add_image_for_relative() -> String {
    let query = r#"
        INSERT INTO image
            (filename, relative_id)
        VALUES ($1, $2)
        ;
    "#;
    query.to_string()
}

pub fn add_note_for_relative() -> String {
    let query = r#"
        INSERT INTO note
            (relative_id, text)
        VALUES
            ($1, $2)
    ;"#;
    query.to_string()
}

pub fn get_images_for_relative() -> String {
    let query = r#"
        SELECT * FROM image WHERE relative_id = $1
        ORDER BY created_at DESC
        ;
    "#;
    query.to_string()
}
