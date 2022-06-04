/**
 * HTTP Method to administrate namespaces
 */
use ntex::web;

use crate::utils::get_poll_conn;
use crate::repositories::namespace;
use crate::models::{NamespaceCreate, Pool};

use crate::repositories::errors::db_bloking_error;

use super::errors::HttpError;

#[utoipa::path(
  get,
  path = "/namespaces",
  responses(
      (status = 200, description = "Array of namespace", body = NamespaceItem),
  ),
)]
#[web::get("/namespaces")]
pub async fn list(pool: web::types::State<Pool>) -> Result<web::HttpResponse, HttpError> {
    let items = namespace::find_all(pool).await?;

    Ok(web::HttpResponse::Ok().json(&items))
}

#[utoipa::path(
  get,
  path = "/namespaces/{id_or_name}",
  responses(
      (status = 200, description = "Namespace found", body = NamespaceItem),
      (status = 404, description = "Namespace not found"),
  ),
  params(
    ("id_or_name" = String, path, description = "Id or Name of the namespace"),
  )
)]
#[web::get("/namespaces/{id_or_name}")]
pub async fn get_by_id_or_name(
    id_or_name: web::types::Path<String>,
    pool: web::types::State<Pool>,
) -> Result<web::HttpResponse, HttpError> {
    let id = id_or_name.into_inner();
    let conn = get_poll_conn(pool)?;
    let res = web::block(move ||
        namespace::find_by_id_or_name(id, &conn)
    ).await;

    match res {
        Err(err) => {
            eprintln!("error : {:?}", err);
            Err(db_bloking_error(err))
        }
        Ok(namespace) => Ok(web::HttpResponse::Ok().json(&namespace)),
    }
}

#[utoipa::path(
  post,
  path = "/namespaces",
  request_body = NamespaceCreate,
  responses(
    (status = 201, description = "Fresh created namespace", body = NamespaceItem),
    (status = 400, description = "Generic database error"),
    (status = 422, description = "The provided payload is not valid"),
  ),
)]
#[web::post("/namespaces")]
pub async fn create(
    pool: web::types::State<Pool>,
    payload: web::types::Json<NamespaceCreate>,
) -> Result<web::HttpResponse, HttpError> {
    let new_namespace = payload.into_inner();
    let item = namespace::create(new_namespace, pool).await?;

    Ok(web::HttpResponse::Created().json(&item))
}

#[utoipa::path(
    delete,
    path = "/namespaces/{id_or_name}",
    responses(
        (status = 200, description = "Database delete response", body = PgDeleteGeneric),
    ),
    params(
        ("id_or_name" = String, path, description = "Id or Name of the namespace"),
    )
)]
#[web::delete("/namespaces/{id_or_name}")]
pub async fn delete_by_id_or_name(
    id_or_name: web::types::Path<String>,
    pool: web::types::State<Pool>,
) -> Result<web::HttpResponse, HttpError> {
    let id = id_or_name.into_inner();
    let conn = get_poll_conn(pool)?;
    let res = web::block(move ||
        namespace::delete_by_id_or_name(id, &conn)
    ).await;
    match res {
        Err(err) => Err(db_bloking_error(err)),
        Ok(json) => Ok(web::HttpResponse::Ok().json(&json)),
    }
}

pub fn ntex_config(config: &mut web::ServiceConfig) {
    config.service(list);
    config.service(create);
    config.service(get_by_id_or_name);
    config.service(delete_by_id_or_name);
}

#[cfg(test)]
mod test_namespace {
    use ntex::web::test::TestServer;
    use serde_json::json;

    use crate::utils::test::*;
    use crate::models::{NamespaceCreate, PgDeleteGeneric};

    use super::ntex_config;

    async fn test_list(srv: &TestServer) -> TestReturn {
        let resp = srv
            .get("/namespaces")
            .send()
            .await?;

        assert!(resp.status().is_success());
        Ok(())
    }

    async fn test_create(srv: &TestServer) -> TestReturn {
        let new_namespace = NamespaceCreate {
            name: String::from("default"),
        };

        let resp = srv
        .post("/namespaces")
        .send_json(&new_namespace)
        .await?;

        println!("{:?}", resp);
        assert!(resp.status().is_success());
        Ok(())
    }

    async fn test_fail_create(srv: &TestServer) -> TestReturn {
        let resp = srv
        .post("/namespaces")
        .send_json(&json!({
            "name": 1,
        })).await?;

        assert!(resp.status().is_client_error());

        let resp = srv
        .post("/namespaces")
        .send().await?;

        assert!(resp.status().is_client_error());
        Ok(())
    }

    async fn test_get_by_id(srv: &TestServer) -> TestReturn {
        let resp = srv
        .get(format!("/namespaces/{name}", name = "default"))
        .send()
        .await?;

        assert!(resp.status().is_success());
        Ok(())
    }

    async fn test_delete(srv: &TestServer) -> TestReturn {
        let mut resp = srv
        .delete(format!("/namespaces/{name}", name = "default"))
        .send()
        .await?;

        let body = resp.json::<PgDeleteGeneric>().await?;
        assert_eq!(body.count, 1);
        assert!(resp.status().is_success());
        Ok(())
    }

    #[ntex::test]
    async fn main() -> TestReturn {
        let srv = generate_server(ntex_config);

        test_fail_create(&srv).await?;
        test_create(&srv).await?;
        test_get_by_id(&srv).await?;
        test_list(&srv).await?;
        test_delete(&srv).await?;
        Ok(())
    }
}
