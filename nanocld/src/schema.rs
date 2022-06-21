table! {
    use diesel::sql_types::*;
    use crate::models::exports::*;

    cargo_ports (key) {
        key -> Varchar,
        cargo_key -> Varchar,
        from -> Int4,
        to -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::exports::*;

    cargos (key) {
        key -> Varchar,
        name -> Varchar,
        image_name -> Varchar,
        network_name -> Varchar,
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

    git_repositories (name) {
        name -> Varchar,
        url -> Varchar,
        default_branch -> Varchar,
        source -> Git_repository_source_type,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::exports::*;

    git_repository_branches (name) {
        name -> Varchar,
        repository_name -> Varchar,
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
    cargo_ports,
    cargos,
    cluster_networks,
    clusters,
    git_repositories,
    git_repository_branches,
    namespaces,
);
