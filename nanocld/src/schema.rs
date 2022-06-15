table! {
    use diesel::sql_types::*;
    use crate::models::exports::*;

    cargos (id) {
        id -> Uuid,
        namespace -> Varchar,
        name -> Varchar,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::exports::*;

    cluster_networks (id) {
        id -> Uuid,
        name -> Varchar,
        cluster_id -> Uuid,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::exports::*;

    clusters (id) {
        id -> Uuid,
        name -> Varchar,
        namespace -> Varchar,
        gen_id -> Varchar,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::exports::*;

    git_repositories (id) {
        id -> Uuid,
        name -> Varchar,
        url -> Varchar,
        token -> Nullable<Varchar>,
        source -> Git_repository_source_type,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::exports::*;

    git_repository_branches (id) {
        id -> Uuid,
        name -> Varchar,
        repository_id -> Uuid,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::exports::*;

    namespaces (id) {
        id -> Uuid,
        name -> Varchar,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::exports::*;

    users (id) {
        id -> Uuid,
        name -> Varchar,
        passwd -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    cargos,
    cluster_networks,
    clusters,
    git_repositories,
    git_repository_branches,
    namespaces,
    users,
);
