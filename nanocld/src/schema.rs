table! {
    use diesel::sql_types::*;

    cargos (id) {
        id -> Uuid,
        namespace -> Varchar,
        name -> Varchar,
    }
}

table! {
    use diesel::sql_types::*;

    cluster_networks (key) {
        key -> Varchar,
        name -> Varchar,
        docker_network_id -> Varchar,
        cluster_key -> Varchar,
    }
}

table! {
    use diesel::sql_types::*;

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

    git_repository_branches (id) {
        id -> Uuid,
        name -> Varchar,
        repository_id -> Uuid,
    }
}

table! {
    use diesel::sql_types::*;

    namespaces (name) {
        name -> Varchar,
    }
}

table! {
    use diesel::sql_types::*;

    users (id) {
        id -> Uuid,
        name -> Varchar,
        passwd -> Varchar,
    }
}

joinable!(cluster_networks -> clusters (cluster_key));

allow_tables_to_appear_in_same_query!(
  cargos,
  cluster_networks,
  clusters,
  git_repositories,
  git_repository_branches,
  namespaces,
  users,
);
