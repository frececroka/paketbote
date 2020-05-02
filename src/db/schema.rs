table! {
    account (id) {
        id -> Int4,
        name -> Varchar,
        salt -> Varchar,
        hashed_password -> Varchar,
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
        created -> Timestamp,
        repo_id -> Int4,
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
    repo_add (id) {
        id -> Int4,
        package_id -> Int4,
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

joinable!(package -> repo (repo_id));
joinable!(repo -> account (owner_id));
joinable!(repo_add -> package (package_id));
joinable!(token -> account (account_id));

allow_tables_to_appear_in_same_query!(
    account,
    package,
    repo,
    repo_add,
    token,
);
