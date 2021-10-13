table! {
    bind_environment_keyvalue (id) {
        id -> Int4,
        fk_environment -> Int4,
        fk_keyvalue -> Int4,
    }
}

table! {
    environment (id) {
        id -> Int4,
        sk -> Bpchar,
        hash_keyvalue -> Bpchar,
        best_before -> Nullable<Int4>,
    }
}

table! {
    keyvalue (id) {
        id -> Int4,
        key -> Text,
        value -> Text,
    }
}

table! {
    project (id) {
        id -> Int4,
        sk -> Bpchar,
        identifier -> Varchar,
        human_name -> Varchar,
    }
}

table! {
    run_identifier (id) {
        id -> Int4,
        sk -> Bpchar,
        client_identifier -> Bpchar,
        created -> Int8,
        fk_project -> Int4,
    }
}

table! {
    test_case (id) {
        id -> Int4,
        sk -> Bpchar,
        name -> Text,
        fk_test_case_class -> Int4,
        fk_test_suite -> Int4,
    }
}

table! {
    test_case_class (id) {
        id -> Int4,
        name -> Text,
    }
}

table! {
    test_case_error (id) {
        id -> Int4,
        fk_test_case -> Int4,
        time -> Nullable<Float4>,
        error_message -> Nullable<Text>,
        error_type -> Nullable<Text>,
        error_description -> Nullable<Text>,
        system_out -> Nullable<Text>,
        system_err -> Nullable<Text>,
        fk_test_file_run -> Int4,
    }
}

table! {
    test_case_failure (id) {
        id -> Int4,
        fk_test_case -> Int4,
        time -> Nullable<Float4>,
        failure_message -> Nullable<Text>,
        failure_type -> Nullable<Text>,
        failure_description -> Nullable<Text>,
        system_out -> Nullable<Text>,
        system_err -> Nullable<Text>,
        fk_test_file_run -> Int4,
    }
}

table! {
    test_case_pass (id) {
        id -> Int4,
        fk_test_case -> Int4,
        time -> Nullable<Float4>,
        fk_test_file_run -> Int4,
    }
}

table! {
    test_case_skipped (id) {
        id -> Int4,
        fk_test_case -> Int4,
        time -> Nullable<Float4>,
        skipped_message -> Nullable<Text>,
        fk_test_file_run -> Int4,
    }
}

table! {
    test_file (id) {
        id -> Int4,
        directory -> Varchar,
        file_name -> Varchar,
    }
}

table! {
    test_file_run (id) {
        id -> Int4,
        sk -> Bpchar,
        fk_test_file -> Int4,
        fk_test_run -> Int4,
    }
}

table! {
    test_run (id) {
        id -> Int4,
        sk -> Bpchar,
        created -> Int8,
        fk_run_identifier -> Int4,
        fk_environment -> Int4,
    }
}

table! {
    test_suite (id) {
        id -> Int4,
        name -> Text,
    }
}

joinable!(bind_environment_keyvalue -> environment (fk_environment));
joinable!(bind_environment_keyvalue -> keyvalue (fk_keyvalue));
joinable!(run_identifier -> project (fk_project));
joinable!(test_case -> test_case_class (fk_test_case_class));
joinable!(test_case -> test_suite (fk_test_suite));
joinable!(test_case_error -> test_case (fk_test_case));
joinable!(test_case_error -> test_file_run (fk_test_file_run));
joinable!(test_case_failure -> test_case (fk_test_case));
joinable!(test_case_failure -> test_file_run (fk_test_file_run));
joinable!(test_case_pass -> test_case (fk_test_case));
joinable!(test_case_pass -> test_file_run (fk_test_file_run));
joinable!(test_case_skipped -> test_case (fk_test_case));
joinable!(test_case_skipped -> test_file_run (fk_test_file_run));
joinable!(test_file_run -> test_file (fk_test_file));
joinable!(test_file_run -> test_run (fk_test_run));
joinable!(test_run -> environment (fk_environment));
joinable!(test_run -> run_identifier (fk_run_identifier));

allow_tables_to_appear_in_same_query!(
    bind_environment_keyvalue,
    environment,
    keyvalue,
    project,
    run_identifier,
    test_case,
    test_case_class,
    test_case_error,
    test_case_failure,
    test_case_pass,
    test_case_skipped,
    test_file,
    test_file_run,
    test_run,
    test_suite,
);
