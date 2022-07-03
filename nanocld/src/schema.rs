table! {
    use crate::models::exports::*;

    cargo_environnements (key) {
        key -> Varchar,
        cargo_key -> Varchar,
        name -> Varchar,
        value -> Varchar,
    }
}

table! {
    use crate::models::exports::*;

    cargo_proxy_configs (cargo_key) {
        cargo_key -> Varchar,
        domain_name -> Varchar,
        host_ip -> Varchar,
        target_port -> Int4,
    }
}

table! {
    use crate::models::exports::*;

    cargoes (key) {
        key -> Varchar,
        name -> Varchar,
        image_name -> Varchar,
        namespace_name -> Varchar,
    }
}

table! {
    use crate::models::exports::*;

    cluster_cargoes (key) {
        key -> Varchar,
        cargo_key -> Varchar,
        cluster_key -> Varchar,
        network_key -> Varchar,
    }
}

table! {
    use crate::models::exports::*;

    cluster_networks (key) {
        key -> Varchar,
        name -> Varchar,
        namespace -> Varchar,
        docker_network_id -> Varchar,
        cluster_key -> Varchar,
    }
}

table! {
    use crate::models::exports::*;

    cluster_variables (key) {
        key -> Varchar,
        cluster_key -> Varchar,
        name -> Varchar,
        value -> Varchar,
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

table! {
    use crate::models::exports::*;

    nginx_templates (name) {
        name -> Varchar,
        content -> Text,
    }
}

joinable!(cargo_proxy_configs -> cargoes (cargo_key));
joinable!(cargoes -> namespaces (namespace_name));
joinable!(cluster_cargoes -> cargoes (cargo_key));
joinable!(cluster_cargoes -> cluster_networks (network_key));
joinable!(cluster_cargoes -> clusters (cluster_key));
joinable!(cluster_networks -> clusters (cluster_key));

allow_tables_to_appear_in_same_query!(
    cargo_environnements,
    cargo_proxy_configs,
    cargoes,
    cluster_cargoes,
    cluster_networks,
    cluster_variables,
    clusters,
    git_repositories,
    git_repository_branches,
    namespaces,
    nginx_templates,
);
