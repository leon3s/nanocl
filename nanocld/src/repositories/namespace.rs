use ntex::web;
use uuid::Uuid;
use diesel::prelude::*;

use crate::controllers::errors::HttpError;
use crate::models::{NamespaceCreate, NamespaceItem, PgDeleteGeneric, Pool};
use crate::utils::get_poll_conn;

use super::errors::db_bloking_error;

pub async fn create(
    item: NamespaceCreate,
    pool: web::types::State<Pool>,
) -> Result<NamespaceItem, HttpError> {
    use crate::schema::namespaces::dsl::*;

    let conn = get_poll_conn(pool)?;
    let res = web::block(move || {
        let item = NamespaceItem {
            id: Uuid::new_v4(),
            name: item.name,
        };
        diesel::insert_into(namespaces)
        .values(&item)
        .execute(&conn)?;
        Ok(item)
    }).await;

    match res {
        Err(err) => Err(db_bloking_error(err)),
        Ok(item) => Ok(item),
    }
}

pub async fn find_all(
    pool: web::types::State<Pool>,
) -> Result<Vec<NamespaceItem>, HttpError> {
    use crate::schema::namespaces::dsl::*;
    
    let conn = get_poll_conn(pool)?;
    let res = web::block(move || {
        namespaces.load::<NamespaceItem>(&conn)
    }).await;

    match res {
        Err(err) => Err(db_bloking_error(err)),
        Ok(items) => Ok(items)
    }
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
