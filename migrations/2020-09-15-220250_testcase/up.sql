PRAGMA foreign_keys = ON;
/*
 Typically map 1:1 to a git repo
 */
CREATE TABLE project (
    id INTEGER PRIMARY KEY NOT NULL,
    sk CHARACTER(32) UNIQUE NOT NULL,
    -- identifier of a project
    identifier VARCHAR UNIQUE NOT NULL,
    -- HUman name for project
    human_name VARCHAR NOT NULL
);
/*
 Each project may have many environment for example a branch and an architecture.
 Environments can be shared with multiple projects.
 */
CREATE TABLE environment (
    id INTEGER PRIMARY KEY NOT NULL,
    sk CHARACTER(32) UNIQUE NOT NULL,
    /*Hash of all key values*/
    hash_keyvalue CHARACTER(32) NOT NULL,
    /* Expire date,
     None for a branch, with a date for a Pull Request */
    best_before INT
);
/*
 Key value pairs make up the compoenents of an environment
 */
CREATE TABLE keyvalue (
    id INTEGER PRIMARY KEY NOT NULL,
    key TEXT NOT NULL,
    value TEXT NOT NULL,
    UNIQUE (key, value) ON CONFLICT ABORT
);
/*
 Environments are defined by the keyvalue that make them up.
 */
CREATE TABLE bind_environment_keyvalue (
    id INTEGER PRIMARY KEY NOT NULL,
    fk_environment INTEGER NOT NULL,
    fk_keyvalue INTEGER NOT NULL,
    FOREIGN KEY (fk_environment) REFERENCES environment (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    FOREIGN KEY (fk_keyvalue) REFERENCES keyvalue (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    UNIQUE (fk_environment, fk_keyvalue) ON CONFLICT ABORT
);
/*
 A run_identifier uniquely describes a run.COLUMN_FORMAT
 
 When you are testing cross platform, for example in macos and linux,
 a run_identifier may be shared by these two environments.
 
 */
CREATE TABLE run_identifier (
    id INTEGER PRIMARY KEY NOT NULL,
    sk CHARACTER(32) UNIQUE NOT NULL,
    /* Client identifier
     for example GIT_COMMIT + CI/CD BUILD_NUMBER
     */
    client_identifier CHARACTER(32) NOT NULL,
    created BigInt NOT NULL,
    fk_project INTEGER NOT NULL,
    FOREIGN KEY (fk_project) REFERENCES project (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    UNIQUE (sk, fk_project) ON CONFLICT ABORT,
    UNIQUE (client_identifier, fk_project) ON CONFLICT ABORT
);
/*
 At least one test_run happens for every run_identifier.
 
 This provides a way to bind environment to run_identifier.
 */
CREATE TABLE test_run (
    id INTEGER PRIMARY KEY NOT NULL,
    sk CHARACTER(32) UNIQUE NOT NULL,
    created BigInt NOT NULL,
    fk_run_identifier INTEGER NOT NULL,
    fk_environment INTEGER NOT NULL,
    FOREIGN KEY (fk_environment) REFERENCES environment (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    FOREIGN KEY (fk_run_identifier) REFERENCES run_identifier (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    UNIQUE (fk_run_identifier, fk_environment) ON CONFLICT ABORT
);
/*
 test_file have one or more test files
 */
CREATE TABLE test_file (
    id INTEGER PRIMARY KEY NOT NULL,
    directory VARCHAR NOT NULL,
    file_name VARCHAR NOT NULL,
    UNIQUE (directory, file_name) ON CONFLICT ABORT
);
/*
 test_run have one or more test files
 */
CREATE TABLE test_file_run (
    id INTEGER PRIMARY KEY NOT NULL,
    sk CHARACTER(32) UNIQUE NOT NULL,
    fk_test_file INTEGER NOT NULL,
    fk_test_run INTEGER NOT NULL,
    FOREIGN KEY (fk_test_file) REFERENCES test_file (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    UNIQUE (fk_test_file, fk_test_run) ON CONFLICT ABORT
);
/*
 One test_suite will exist for each test case
 */
CREATE TABLE test_suite (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT UNIQUE NOT NULL
);
/*
 One test_class will exist for each test case
 */
CREATE TABLE test_case_class (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT UNIQUE NOT NULL
);
/*
 Many tests have the same name and classname and test suite
 */
CREATE TABLE test_case (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    fk_test_case_class INTEGER NOT NULL,
    fk_test_suite INTEGER NOT NULL,
    FOREIGN KEY (fk_test_case_class) REFERENCES test_case_class (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    FOREIGN KEY (fk_test_suite) REFERENCES test_suite (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    UNIQUE (name, fk_test_case_class, fk_test_suite) ON CONFLICT ABORT
);
/*
 Allows grouping of many environments in a single run
 this maybe shared across environments but not projects
 */
CREATE TABLE test_case_pass (
    id INTEGER PRIMARY KEY NOT NULL,
    fk_test_case INTEGER NOT NULL,
    /* Number of seconds to run */
    time REAL,
    fk_test_file_run INTEGER NOT NULL,
    FOREIGN KEY (fk_test_case) REFERENCES test_case (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    FOREIGN KEY (fk_test_file_run) REFERENCES test_file_run (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    UNIQUE (fk_test_case, fk_test_file_run) ON CONFLICT ABORT
);
/*
 Allows grouping of many environments in a single run
 this maybe shared across environments but not projects
 */
CREATE TABLE test_case_skipped (
    id INTEGER PRIMARY KEY NOT NULL,
    fk_test_case INTEGER NOT NULL,
    /* Number of seconds to run */
    time REAL,
    skipped_message TEXT,
    fk_test_file_run INTEGER NOT NULL,
    FOREIGN KEY (fk_test_case) REFERENCES test_case (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    FOREIGN KEY (fk_test_file_run) REFERENCES test_file_run (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    UNIQUE (fk_test_case, fk_test_file_run) ON CONFLICT ABORT
);
/*
 Allows grouping of many environments in a single run
 this maybe shared across environments but not projects
 */
CREATE TABLE test_case_error (
    id INTEGER PRIMARY KEY NOT NULL,
    fk_test_case INTEGER NOT NULL,
    /* Number of seconds to run */
    time REAL,
    error_message TEXT,
    error_type TEXT,
    error_description TEXT,
    system_out TEXT,
    system_err TEXT,
    fk_test_file_run INTEGER NOT NULL,
    FOREIGN KEY (fk_test_case) REFERENCES test_case (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    FOREIGN KEY (fk_test_file_run) REFERENCES test_file_run (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    UNIQUE (fk_test_case, fk_test_file_run) ON CONFLICT ABORT
);
/*
 Allows grouping of many environments in a single run
 this maybe shared across environments but not projects
 */
CREATE TABLE test_case_failure (
    id INTEGER PRIMARY KEY NOT NULL,
    fk_test_case INTEGER NOT NULL,
    /* Number of seconds to run */
    time REAL,
    failure_message TEXT,
    failure_type TEXT,
    failure_description TEXT,
    system_out TEXT,
    system_err TEXT,
    fk_test_file_run INTEGER NOT NULL,
    FOREIGN KEY (fk_test_case) REFERENCES test_case (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    FOREIGN KEY (fk_test_file_run) REFERENCES test_file_run (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    UNIQUE (fk_test_case, fk_test_file_run) ON CONFLICT ABORT
);