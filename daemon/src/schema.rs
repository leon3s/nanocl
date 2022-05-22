table! {
    cargos (id) {
        id -> Uuid,
        namespace -> Varchar,
        name -> Varchar,
    }
}

table! {
    namespaces (id) {
        id -> Uuid,
        name -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    cargos,
    namespaces,
);
