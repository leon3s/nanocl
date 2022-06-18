table! {
    use diesel::sql_types::*;
    use crate::models::exports::*;

    cargos (key) {
        key -> Varchar,
        name -> Varchar,
        image_name -> Varchar,
        network_name -> Varchar,
        repository_name -> Varchar,
        namespace_name -> Varchar,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::exports::*;

    cluster_networks (key) {
        key -> Varchar,
        name -> Varchar,
        docker_network_id -> Varchar,
        cluster_key -> Varchar,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::exports::*;

    clusters (key) {
        key -> Varchar,
        name -> Varchar,
        namespace -> Varchar,
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

    namespaces (name) {
        name -> Varchar,
    }
}

joinable!(cargos -> namespaces (namespace_name));
joinable!(cluster_networks -> clusters (cluster_key));

allow_tables_to_appear_in_same_query!(
    cargos,
    cluster_networks,
    clusters,
    git_repositories,
    git_repository_branches,
    namespaces,
);
