table! {
    account (id) {
        id -> Int4,
        name -> Varchar,
        salt -> Varchar,
        hashed_password -> Varchar,
    }
}

table! {
    job (id) {
        id -> Int4,
        tag -> Varchar,
        spec -> Jsonb,
        worker -> Nullable<Varchar>,
    }
}

table! {
    missing_dep (id) {
        id -> Int4,
        repo_id -> Int4,
        dependency -> Varchar,
    }
}

table! {
    package (id) {
        id -> Int4,
        name -> Varchar,
        version -> Varchar,
        arch -> Varchar,
        size -> Int4,
        archive -> Varchar,
        signature -> Varchar,
        compression -> Varchar,
        created -> Timestamp,
        active -> Bool,
        deleted -> Bool,
        repo_id -> Int4,
    }
}

table! {
    package_depends (id) {
        id -> Int4,
        package_id -> Int4,
        depends -> Varchar,
    }
}

table! {
    package_provides (id) {
        id -> Int4,
        package_id -> Int4,
        provides -> Varchar,
    }
}

table! {
    repo (id) {
        id -> Int4,
        name -> Varchar,
        owner_id -> Int4,
    }
}

table! {
    repo_action (id) {
        id -> Int4,
        package_id -> Int4,
        action -> Varchar,
        worker -> Nullable<Varchar>,
    }
}

table! {
    token (id) {
        id -> Int4,
        name -> Varchar,
        the_token -> Varchar,
        account_id -> Int4,
    }
}

joinable!(missing_dep -> repo (repo_id));
joinable!(package -> repo (repo_id));
joinable!(package_depends -> package (package_id));
joinable!(package_provides -> package (package_id));
joinable!(repo -> account (owner_id));
joinable!(repo_action -> package (package_id));
joinable!(token -> account (account_id));

allow_tables_to_appear_in_same_query!(
    account,
    job,
    missing_dep,
    package,
    package_depends,
    package_provides,
    repo,
    repo_action,
    token,
);
