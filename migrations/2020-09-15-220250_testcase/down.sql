-- This file should undo anything in `up.sql`
DROP TABLE project;
DROP TABLE environment;
DROP TABLE keyvalue;
DROP TABLE bind_environment_keyvalue;
DROP TABLE run_identifier;
DROP TABLE test_run;
DROP TABLE test_case_skipped;
DROP TABLE test_case_error;
DROP TABLE test_case_failure;
DROP TABLE test_case_pass;