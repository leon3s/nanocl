table! {
    cargos (id) {
        id -> Uuid,
        namespace -> Varchar,
        name -> Varchar,
    }
}

table! {
    git_repositories (id) {
        id -> Uuid,
        namespace -> Varchar,
        name -> Varchar,
        url -> Varchar,
        token -> Varchar,
    }
}

table! {
    namespaces (id) {
        id -> Uuid,
        name -> Varchar,
    }
}

table! {
    users (id) {
        id -> Uuid,
        name -> Varchar,
        passwd -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    cargos,
    git_repositories,
    namespaces,
    users,
);
