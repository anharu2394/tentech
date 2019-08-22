use crate::db;
use crate::error::TentechError;
use crate::models::product::Product;
use crate::schema::products;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error};
use std::time::SystemTime;
use uuid::Uuid;

#[derive(Insertable, AsChangeset)]
#[primary_key(uuid)]
#[table_name = "products"]
pub struct NewProduct<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub img: &'a str,
    pub duration: &'a i32,
    pub kind: &'a str,
    pub user_id: &'a i32,
    pub uuid: &'a Uuid,
}

pub fn create(
    conn: &PgConnection,
    title: &str,
    body: &str,
    img: &str,
    duration: &i32,
    kind: &str,
    tags: &Vec<i32>,
    user_id: &i32,
) -> Result<Product, Error> {
    let new_product = &NewProduct {
        title,
        body,
        img,
        duration,
        kind,
        user_id,
        uuid: &Uuid::new_v4(),
    };

    let product = diesel::insert_into(products::table)
        .values(new_product)
        .get_result::<Product>(conn)?;
    db::tags::entry_to_product(conn, product.id, tags.to_vec());
    Ok(product)
}

pub fn update(
    conn: &PgConnection,
    title: &str,
    body: &str,
    img: &str,
    duration: &i32,
    kind: &str,
    tags: &Vec<i32>,
    user_id: &i32,
    uuid: &Uuid,
) -> Result<Product, Error> {
    let new_product = &NewProduct {
        title,
        body,
        img,
        duration,
        kind,
        user_id,
        uuid,
    };

    let product = diesel::update(products::table)
        .set(new_product)
        .get_result::<Product>(conn)?;
    db::tags::delete_by_product_id(conn, product.id)?;
    db::tags::entry_to_product(conn, product.id, tags.to_vec())?;
    Ok(product)
}

pub fn find(conn: &PgConnection, id: &Uuid) -> Result<Product, Error> {
    products::table
        .filter(products::uuid.eq(id))
        .first::<Product>(conn)
}

pub fn delete(conn: &PgConnection, id: &Uuid) -> Result<usize, Error> {
    diesel::delete(products::table.filter(products::uuid.eq(id))).execute(conn)
}
