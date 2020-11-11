table! {
    bind_enviroment_keyvalue (id) {
        id -> Integer,
        fk_enviroment -> Integer,
        fk_keyvalue -> Integer,
    }
}

table! {
    enviroment (id) {
        id -> Integer,
        sk -> Text,
        hash_keyvalue -> Text,
        best_before -> Nullable<Integer>,
        fk_project -> Integer,
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
        identiifier -> Text,
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
        classname -> Text,
        time -> Nullable<Integer>,
        skipped_message -> Nullable<Text>,
        error_message -> Nullable<Text>,
        error_type -> Nullable<Text>,
        error_description -> Nullable<Text>,
        failure_message -> Nullable<Text>,
        failure_type -> Nullable<Text>,
        failure_description -> Nullable<Text>,
        system_out -> Nullable<Text>,
        system_err -> Nullable<Text>,
        fk_test_run -> Integer,
    }
}

table! {
    test_run (id) {
        id -> Integer,
        sk -> Text,
        client_identifier -> Text,
        created -> BigInt,
        fk_run_identifier -> Integer,
        fk_enviroment -> Integer,
    }
}

joinable!(bind_enviroment_keyvalue -> enviroment (fk_enviroment));
joinable!(bind_enviroment_keyvalue -> keyvalue (fk_keyvalue));
joinable!(enviroment -> project (fk_project));
joinable!(run_identifier -> project (fk_project));
joinable!(test_case -> test_run (fk_test_run));
joinable!(test_run -> enviroment (fk_enviroment));
joinable!(test_run -> run_identifier (fk_run_identifier));

allow_tables_to_appear_in_same_query!(
    bind_enviroment_keyvalue,
    enviroment,
    keyvalue,
    project,
    run_identifier,
    test_case,
    test_run,
);
