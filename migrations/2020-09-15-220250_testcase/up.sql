PRAGMA foreign_keys = ON;
/*
 Typically map 1:1 to a git repo
 */
CREATE TABLE project (
    id INTEGER PRIMARY KEY NOT NULL,
    sk CHARACTER(32) NOT NULL,
    -- identiifier of a project
    identiifier VARCHAR NOT NULL,
    -- HUman name for project
    human_name VARCHAR NOT NULL
);
/*
 Each project may have many enviroment for example a branch and an architecture.
 */
CREATE TABLE enviroment (
    id INTEGER PRIMARY KEY NOT NULL,
    sk CHARACTER(32) NOT NULL,
    /*Hash of all key values*/
    hash_keyvalue CHARACTER(32) UNIQUE NOT NULL,
    /* Expire date,
     None for a branch, with a date for a Pull Request */
    best_before INT,
    fk_project INTEGER NOT NULL,
    FOREIGN KEY (fk_project) REFERENCES project (id) ON DELETE CASCADE ON UPDATE NO ACTION
);
/*
 Key value pairs make up the compoenents of an enviroment
 */
CREATE TABLE keyvalue (
    id INTEGER PRIMARY KEY NOT NULL,
    key TEXT NOT NULL,
    value TEXT NOT NULL,
    UNIQUE (key, value) ON CONFLICT IGNORE
);
/*
 Enviroments are defined by the keyvalue that make them up.
 */
CREATE TABLE bind_enviroment_keyvalue (
    id INTEGER PRIMARY KEY NOT NULL,
    fk_enviroment INTEGER NOT NULL,
    fk_keyvalue INTEGER NOT NULL,
    FOREIGN KEY (fk_enviroment) REFERENCES enviroment (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    FOREIGN KEY (fk_keyvalue) REFERENCES keyvalue (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    UNIQUE (fk_enviroment, fk_keyvalue) ON CONFLICT ABORT
);
/*
 Allows grouping of many enviroments in a single run
 this maybe shared across enviroments but not projects
 */
CREATE TABLE run_identifier (
    id INTEGER PRIMARY KEY NOT NULL,
    sk CHARACTER(32) NOT NULL,
    /* Client identifier
     for example GIT_COMMIT + CI/CD BUILD_NUMBER
     */
    client_identifier CHARACTER(32) NOT NULL,
    created INT,
    fk_project INTEGER NOT NULL,
    FOREIGN KEY (fk_project) REFERENCES project (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    UNIQUE (sk, fk_project) ON CONFLICT ABORT
);
/*
 Allows grouping of many enviroments in a single run
 this maybe shared across enviroments but not projects
 */
CREATE TABLE test_run (
    id INTEGER PRIMARY KEY NOT NULL,
    sk CHARACTER(32) NOT NULL,
    /* Client identifier
     for example GIT_COMMIT + CI/CD BUILD_NUMBER
     */
    client_identifier CHARACTER(32) NOT NULL,
    created INT,
    fk_run_identifier INTEGER NOT NULL,
    fk_enviroment INTEGER NOT NULL,
    FOREIGN KEY (fk_enviroment) REFERENCES enviroment (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    FOREIGN KEY (fk_run_identifier) REFERENCES run_identifier (id) ON DELETE CASCADE ON UPDATE NO ACTION
);
/*
 Allows grouping of many enviroments in a single run
 this maybe shared across enviroments but not projects
 */
CREATE TABLE test_case (
    id INTEGER PRIMARY KEY NOT NULL,
    /* Client identifier
     for example GIT_COMMIT + CI/CD BUILD_NUMBER
     */
    name TEXT NOT NULL,
    classname TEXT NOT NULL,
    /* Number of seconds to run */
    time INTEGER,
    skipped_message TEXT,
    error_message TEXT,
    error_type TEXT,
    error_description TEXT,
    failure_message TEXT,
    failure_type TEXT,
    failure_description TEXT,
    system_out TEXT,
    system_err TEXT,
    fk_test_run INTEGER NOT NULL,
    FOREIGN KEY (fk_test_run) REFERENCES test_run (id) ON DELETE CASCADE ON UPDATE NO ACTION
);