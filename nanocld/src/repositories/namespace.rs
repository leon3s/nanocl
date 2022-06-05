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

pub async fn find_by_id_or_name(
    id_or_name: String,
    pool: web::types::State<Pool>,
) -> Result<NamespaceItem, HttpError> {
    use crate::schema::namespaces::dsl::*;
    let conn = get_poll_conn(pool)?;

    let res = match Uuid::parse_str(&id_or_name) {
        Err(_) => web::block(move || {
            namespaces.filter(name.eq(id_or_name))
            .get_result::<NamespaceItem>(&conn)
        }).await,
        Ok(uuid) => web::block(move || {
            namespaces.filter(id.eq(uuid))
            .get_result::<NamespaceItem>(&conn)
        }).await,
    };

    match res {
        Err(err) => Err(db_bloking_error(err)),
        Ok(item) => Ok(item),
    }
}

pub async fn delete_by_id_or_name(
    id_or_name: String,
    pool: web::types::State<Pool>,
) -> Result<PgDeleteGeneric, HttpError> {
    use crate::schema::namespaces::dsl::*;
    
    let conn = get_poll_conn(pool)?;

    let res = match Uuid::parse_str(&id_or_name) {
        Err(_) => web::block(move || {
            diesel::delete(namespaces.filter(name.eq(id_or_name))).execute(&conn)
        }).await,
        Ok(uuid) => web::block(move || {
            diesel::delete(namespaces.filter(id.eq(uuid))).execute(&conn)
        }).await,
    };

    match res {
        Err(err) => Err(db_bloking_error(err)),
        Ok(result) => Ok(PgDeleteGeneric { count: result }),
    }
}
