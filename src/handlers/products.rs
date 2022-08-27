use actix_web::{delete, get, post, put, web, HttpResponse};

use crate::errors::server_error::ServerError;
use crate::handlers::{pg_pool_handler, LoggedUser};
use crate::models::product::{NewProduct, Product, ProductsList};

use crate::db_connection::PgPool;

// Add pool handlers

#[get("")]
pub async fn index(
    _user: LoggedUser,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ServerError> {
    let pool = pg_pool_handler(pool)?;
    Ok(HttpResponse::Ok().json(ProductsList::list(&pool)))
}

// Create Product
#[post("")]
pub async fn create(
    _user: LoggedUser,
    new_product: web::Json<NewProduct>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ServerError> {
    let pool = pg_pool_handler(pool)?;

    new_product
        .create(&pool)
        .map(|product| HttpResponse::Created().json(product))
        .map_err(|err| ServerError::InternalServerError(err.to_string()))
}

// Get a product by id
#[get("/{id}")]
pub async fn get(
    _user: LoggedUser,
    id: web::Path<i32>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ServerError> {
    let pool = pg_pool_handler(pool)?;
    Product::find(&id.into_inner(), &pool)
        .map(|product| HttpResponse::Ok().json(product))
        .map_err(|err| ServerError::InternalServerError(err.to_string()))
}

// Delete a product by id
#[delete("/{id}")]
pub async fn destroy(
    _user: LoggedUser,
    id: web::Path<i32>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ServerError> {
    let pool = pg_pool_handler(pool)?;
    Product::destroy(&id.into_inner(), &pool)
        .map(|_| HttpResponse::NoContent().json(()))
        .map_err(|err| ServerError::InternalServerError(err.to_string()))
}

// Update a product by id
#[put("/{id}")]
async fn update(
    _user: LoggedUser,
    id: web::Path<i32>,
    new_product: web::Json<NewProduct>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ServerError> {
    let pool = pg_pool_handler(pool)?;
    Product::update(&id.into_inner(), &new_product, &pool)
        .map(|product| HttpResponse::Ok().json(product))
        .map_err(|err| ServerError::InternalServerError(err.to_string()))
}
