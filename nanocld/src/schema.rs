table! {
    use crate::models::exports::*;

    cargo_ports (key) {
        key -> Varchar,
        cargo_key -> Varchar,
        from -> Int4,
        to -> Int4,
    }
}

table! {
    use crate::models::exports::*;

    cargo_proxy_configs (cargo_key) {
        cargo_key -> Varchar,
        domain_name -> Varchar,
        host_ip -> Varchar,
    }
}

table! {
    use crate::models::exports::*;

    cargos (key) {
        key -> Varchar,
        name -> Varchar,
        image_name -> Varchar,
        network_name -> Nullable<Varchar>,
        domain_name -> Nullable<Varchar>,
        host_ip -> Nullable<Varchar>,
        namespace_name -> Varchar,
    }
}

table! {
    use crate::models::exports::*;

    cluster_networks (key) {
        key -> Varchar,
        name -> Varchar,
        docker_network_id -> Varchar,
        cluster_key -> Varchar,
    }
}

table! {
    use crate::models::exports::*;

    clusters (key) {
        key -> Varchar,
        name -> Varchar,
        namespace -> Varchar,
    }
}

table! {
    use crate::models::exports::*;

    git_repositories (name) {
        name -> Varchar,
        url -> Varchar,
        default_branch -> Varchar,
        source -> Git_repository_source_type,
    }
}

table! {
    use crate::models::exports::*;

    git_repository_branches (key) {
        key -> Varchar,
        name -> Varchar,
        last_commit_sha -> Varchar,
        repository_name -> Varchar,
    }
}

table! {
    use crate::models::exports::*;

    namespaces (name) {
        name -> Varchar,
    }
}

joinable!(cargo_proxy_configs -> cargos (cargo_key));
joinable!(cargos -> namespaces (namespace_name));
joinable!(cluster_networks -> clusters (cluster_key));

allow_tables_to_appear_in_same_query!(
    cargo_ports,
    cargo_proxy_configs,
    cargos,
    cluster_networks,
    clusters,
    git_repositories,
    git_repository_branches,
    namespaces,
);
