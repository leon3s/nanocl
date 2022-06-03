use diesel::prelude::*;
use uuid::Uuid;

use crate::models::{ClusterCreate, ClusterItem};

pub fn create_for_namespace(
    nsp: String,
    item: ClusterCreate,
    conn: &PgConnection,
) -> Result<ClusterItem, diesel::result::Error> {
    use crate::schema::clusters::dsl::*;

    let new_cluster = ClusterItem {
        id: Uuid::new_v4(),
        name: item.name,
        namespace: nsp,
    };

    diesel::insert_into(clusters)
        .values(&new_cluster)
        .execute(conn)?;

    Ok(new_cluster)
}

pub fn find_by_namespace(
    nsp: String,
    conn: &PgConnection,
) -> Result<Vec<ClusterItem>, diesel::result::Error> {
    use crate::schema::clusters::dsl::*;

    let items = clusters
        .filter(namespace.eq(nsp))
        .load::<ClusterItem>(conn)?;
    Ok(items)
}
