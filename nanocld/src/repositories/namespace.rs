use diesel::prelude::*;
use uuid::Uuid;

use crate::models::{NamespaceCreate, NamespaceItem, PgDeleteGeneric};

pub fn create(
    item: NamespaceCreate,
    conn: &PgConnection,
) -> Result<NamespaceItem, diesel::result::Error> {
    use crate::schema::namespaces::dsl::*;

    let new_namespace = NamespaceItem {
        id: Uuid::new_v4(),
        name: item.name,
    };

    diesel::insert_into(namespaces)
        .values(&new_namespace)
        .execute(conn)?;
    Ok(new_namespace)
}

pub fn find_all(conn: &PgConnection) -> Result<Vec<NamespaceItem>, diesel::result::Error> {
    use crate::schema::namespaces::dsl::*;

    let items = namespaces.load::<NamespaceItem>(conn)?;
    Ok(items)
}

pub fn find_by_id_or_name(
    id_or_name: String,
    conn: &PgConnection,
) -> Result<NamespaceItem, diesel::result::Error> {
    use crate::schema::namespaces::dsl::*;

    match Uuid::parse_str(&id_or_name) {
        Err(_) => {
            let result = namespaces
                .filter(name.eq(id_or_name))
                .get_result::<NamespaceItem>(conn)?;
            Ok(result)
        }
        Ok(uuid) => {
            let result = namespaces
                .filter(id.eq(uuid))
                .get_result::<NamespaceItem>(conn)?;
            Ok(result)
        }
    }
}

pub fn delete_by_id_or_name(
    id_or_name: String,
    conn: &PgConnection,
) -> Result<PgDeleteGeneric, diesel::result::Error> {
    use crate::schema::namespaces::dsl::*;

    match Uuid::parse_str(&id_or_name) {
        Err(_) => {
            let result = diesel::delete(namespaces.filter(name.eq(id_or_name))).execute(conn)?;
            println!("delete result {:?}", result);
            Ok(PgDeleteGeneric { count: result })
        }
        Ok(uuid) => {
            let result = diesel::delete(namespaces.filter(id.eq(uuid))).execute(conn)?;
            println!("delete result {:?}", result);
            Ok(PgDeleteGeneric { count: result })
        }
    }
}
