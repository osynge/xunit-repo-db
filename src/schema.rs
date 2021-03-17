table! {
    bind_environment_keyvalue (id) {
        id -> Integer,
        fk_environment -> Integer,
        fk_keyvalue -> Integer,
    }
}

table! {
    environment (id) {
        id -> Integer,
        sk -> Text,
        hash_keyvalue -> Text,
        best_before -> Nullable<Integer>,
    }
}

table! {
    keyvalue (id) {
        id -> Integer,
        key -> Text,
        value -> Text,
    }
}

table! {
    project (id) {
        id -> Integer,
        sk -> Text,
        identifier -> Text,
        human_name -> Text,
    }
}

table! {
    run_identifier (id) {
        id -> Integer,
        sk -> Text,
        client_identifier -> Text,
        created -> BigInt,
        fk_project -> Integer,
    }
}

table! {
    test_case (id) {
        id -> Integer,
        name -> Text,
        fk_test_case_class -> Integer,
        fk_test_suite -> Integer,
    }
}

table! {
    test_case_class (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    test_case_error (id) {
        id -> Integer,
        fk_test_case -> Integer,
        time -> Nullable<Float>,
        error_message -> Nullable<Text>,
        error_type -> Nullable<Text>,
        error_description -> Nullable<Text>,
        system_out -> Nullable<Text>,
        system_err -> Nullable<Text>,
        fk_test_file_run -> Integer,
    }
}

table! {
    test_case_failure (id) {
        id -> Integer,
        fk_test_case -> Integer,
        time -> Nullable<Float>,
        failure_message -> Nullable<Text>,
        failure_type -> Nullable<Text>,
        failure_description -> Nullable<Text>,
        system_out -> Nullable<Text>,
        system_err -> Nullable<Text>,
        fk_test_file_run -> Integer,
    }
}

table! {
    test_case_pass (id) {
        id -> Integer,
        fk_test_case -> Integer,
        time -> Nullable<Float>,
        fk_test_file_run -> Integer,
    }
}

table! {
    test_case_skipped (id) {
        id -> Integer,
        fk_test_case -> Integer,
        time -> Nullable<Float>,
        skipped_message -> Nullable<Text>,
        fk_test_file_run -> Integer,
    }
}

table! {
    test_file (id) {
        id -> Integer,
        directory -> Text,
        file_name -> Text,
    }
}

table! {
    test_file_run (id) {
        id -> Integer,
        sk -> Text,
        fk_test_file -> Integer,
        fk_test_run -> Integer,
    }
}

table! {
    test_run (id) {
        id -> Integer,
        sk -> Text,
        created -> BigInt,
        fk_run_identifier -> Integer,
        fk_environment -> Integer,
    }
}

table! {
    test_suite (id) {
        id -> Integer,
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
