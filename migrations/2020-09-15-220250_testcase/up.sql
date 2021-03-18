PRAGMA foreign_keys = ON;

/*
 Typically map 1:1 to a git repo

 @sk - (Surrogate Key) Allows grouping of many environments in a single run
 this maybe shared across environments but not projects
 @identifier - Project identifier, for example "test-project"
 @human_name - Human name for the project, for example "Test Project"
 */
CREATE TABLE project (
    id INTEGER PRIMARY KEY NOT NULL,
    sk CHARACTER(32) UNIQUE NOT NULL,
    identifier VARCHAR UNIQUE NOT NULL,
    human_name VARCHAR NOT NULL
);

/*
 Each project can have many environments, for example a branch and an
 architecture. Environments can be shared with multiple projects.

 @hash_keyvalue - A hash of the key/value???
 @best_before - Expiry date of the environment. Useful for pull requests.???
 */
CREATE TABLE environment (
    id INTEGER PRIMARY KEY NOT NULL,
    sk CHARACTER(32) UNIQUE NOT NULL,
    hash_keyvalue CHARACTER(32) NOT NULL,
    best_before INT
);

/*
 Key value pairs make up the components of an environment.

 @key - The name of the environment, for example 'BRANCH'
 @value - The value of the environment key, for example branch name 'feature_42'
 */
CREATE TABLE keyvalue (
    id INTEGER PRIMARY KEY NOT NULL,
    key TEXT NOT NULL,
    value TEXT NOT NULL,
    UNIQUE (key, value) ON CONFLICT ABORT
);

/*
 Environments are defined by the keyvalue that make them up.

 ???For example "macos-main" as environment-branch?
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
 A run_identifier uniquely describes a run for a specific project.

 When you are testing cross platform, for example in macos and linux,
 a run_identifier may be shared by these two environments.

 @client_identifier - Specifies an identifier for the run. For example GIT_COMMIT + CI/CD BUILD_NUMBER.
 @created - The time when this run was started.
 */
CREATE TABLE run_identifier (
    id INTEGER PRIMARY KEY NOT NULL,
    sk CHARACTER(32) UNIQUE NOT NULL,
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
 ???why not have one test_file_path???

 @directory - The directory of the test file
 @file_name - The name of the test file
 */
CREATE TABLE test_file (
    id INTEGER PRIMARY KEY NOT NULL,
    directory VARCHAR NOT NULL,
    file_name VARCHAR NOT NULL,
    UNIQUE (directory, file_name) ON CONFLICT ABORT
);

/*
 A test run can have one or more test files.
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
 One test suite will exist for each test case.

 @name - The name of the test suite
 */
CREATE TABLE test_suite (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT UNIQUE NOT NULL
);

/*
 One test class will exist for each test case.

 @name - The name of the test case class
 */
CREATE TABLE test_case_class (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT UNIQUE NOT NULL
);

/*
 Many tests have the same name, class name and test suite.

 @name - The name of the test case
 */
CREATE TABLE test_case (
    id INTEGER PRIMARY KEY NOT NULL,
    sk CHARACTER(32) UNIQUE NOT NULL,
    name TEXT NOT NULL,
    fk_test_case_class INTEGER NOT NULL,
    fk_test_suite INTEGER NOT NULL,
    FOREIGN KEY (fk_test_case_class) REFERENCES test_case_class (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    FOREIGN KEY (fk_test_suite) REFERENCES test_suite (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    UNIQUE (name, fk_test_case_class, fk_test_suite) ON CONFLICT ABORT
);

/*
 A test that was successful and passed.

 @time - Number of seconds that the test run
 */
CREATE TABLE test_case_pass (
    id INTEGER PRIMARY KEY NOT NULL,
    fk_test_case INTEGER NOT NULL,
    time REAL,
    fk_test_file_run INTEGER NOT NULL,
    FOREIGN KEY (fk_test_case) REFERENCES test_case (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    FOREIGN KEY (fk_test_file_run) REFERENCES test_file_run (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    UNIQUE (fk_test_case, fk_test_file_run) ON CONFLICT ABORT
);

/*
 A skipped test indicates that it was intentionally not run.

 @skipped_message - The message or description why the test was skipped.
 @time - Number of seconds that the test run
 */
CREATE TABLE test_case_skipped (
    id INTEGER PRIMARY KEY NOT NULL,
    fk_test_case INTEGER NOT NULL,
    time REAL,
    skipped_message TEXT,
    fk_test_file_run INTEGER NOT NULL,
    FOREIGN KEY (fk_test_case) REFERENCES test_case (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    FOREIGN KEY (fk_test_file_run) REFERENCES test_file_run (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    UNIQUE (fk_test_case, fk_test_file_run) ON CONFLICT ABORT
);

/*
 ???Allows grouping of many environments in a single run
 this maybe shared across environments but not projects???missing _message and _description

 An error indicates that the test encountered a untypical problem. This could be
 a crash or (uncatched) exception during test run, outside your test assertions/checks.

 @time - Number of seconds that the test run
 @error_message - The error message, for example if a java exception is thrown, the return value of getMessage()
 @error_type - The type of error that occured, for example if a Java exception is thrown, the full class name of the exception
 @error_description - (optional) The detailed description for the error, for example a stack trace
 */
CREATE TABLE test_case_error (
    id INTEGER PRIMARY KEY NOT NULL,
    fk_test_case INTEGER NOT NULL,
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
 ???Allows grouping of many environments in a single run
 this maybe shared across environments but not projects???system_out and system_err should be separate, right and have a reference to test_case???

 A failure indicates that the test failed. It is a condition in which that test failed,
 given the checks, for example `assertEquals`.

 @time - Number of seconds that the test run
 @failure_message - The message specified in the test assert
 @failure_type - The type of the assert
 @failure_description - (optional) The detailed description for the failure, for example a stack trace
 */
CREATE TABLE test_case_failure (
    id INTEGER PRIMARY KEY NOT NULL,
    fk_test_case INTEGER NOT NULL,
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
