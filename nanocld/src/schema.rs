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

    cargoes (key) {
        key -> Varchar,
        namespace_name -> Varchar,
        name -> Varchar,
        image_name -> Varchar,
        binds -> Array<Text>,
        domain -> Nullable<Varchar>,
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
        default_gateway -> Varchar,
        cluster_key -> Varchar,
    }
}

table! {
    use crate::models::exports::*;

    cluster_proxy_configs (cluster_key) {
        cluster_key -> Varchar,
        template -> Array<Text>,
        target_port -> Int4,
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

    nginx_logs (key) {
        key -> Uuid,
        date_gmt -> Timestamptz,
        uri -> Varchar,
        host -> Varchar,
        remote_addr -> Varchar,
        realip_remote_addr -> Varchar,
        server_protocol -> Varchar,
        request_method -> Varchar,
        content_length -> Int8,
        status -> Int4,
        request_time -> Float8,
        body_bytes_sent -> Int8,
        proxy_host -> Nullable<Varchar>,
        upstream_addr -> Nullable<Varchar>,
        query_string -> Nullable<Varchar>,
        request_body -> Nullable<Varchar>,
        content_type -> Nullable<Varchar>,
        http_user_agent -> Nullable<Varchar>,
        http_referrer -> Nullable<Varchar>,
        http_accept_language -> Nullable<Varchar>,
    }
}

table! {
    use crate::models::exports::*;

    nginx_templates (name) {
        name -> Varchar,
        mode -> Nginx_template_modes,
        content -> Text,
    }
}

joinable!(cargoes -> namespaces (namespace_name));
joinable!(cluster_cargoes -> cargoes (cargo_key));
joinable!(cluster_cargoes -> cluster_networks (network_key));
joinable!(cluster_cargoes -> clusters (cluster_key));
joinable!(cluster_networks -> clusters (cluster_key));
joinable!(cluster_proxy_configs -> clusters (cluster_key));

allow_tables_to_appear_in_same_query!(
    cargo_environnements,
    cargoes,
    cluster_cargoes,
    cluster_networks,
    cluster_proxy_configs,
    cluster_variables,
    clusters,
    git_repositories,
    git_repository_branches,
    namespaces,
    nginx_logs,
    nginx_templates,
);
