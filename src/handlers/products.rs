use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};

use crate::errors::user_error::UserError;
use crate::models::product::{NewProduct, Product, ProductsList};

use crate::db_connection::{PgPool, PgPooledConnection};

// Add pool handlers
fn pool_handler(pool: web::Data<PgPool>) -> Result<PgPooledConnection, UserError> {
    pool.get()
        .map_err(|e| UserError::InternalServerError(e.to_string()))
}

#[get("")]
pub async fn index(_req: HttpRequest, pool: web::Data<PgPool>) -> Result<HttpResponse, UserError> {
    let pool = pool_handler(pool)?;
    Ok(HttpResponse::Ok().json(ProductsList::list(&pool)))
}

// Create Product
#[post("")]
pub async fn create(
    new_product: web::Json<NewProduct>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, UserError> {
    let pool = pool_handler(pool)?;

    new_product
        .create(&pool)
        .map(|product| HttpResponse::Created().json(product))
        .map_err(|err| UserError::InternalServerError(err.to_string()))
}

// Get a product by id
#[get("/{id}")]
pub async fn get(id: web::Path<i32>, pool: web::Data<PgPool>) -> Result<HttpResponse, UserError> {
    let pool = pool_handler(pool)?;
    Product::find(&id.into_inner(), &pool)
        .map(|product| HttpResponse::Ok().json(product))
        .map_err(|err| UserError::InternalServerError(err.to_string()))
}

// Delete a product by id
#[delete("/{id}")]
pub async fn destroy(
    id: web::Path<i32>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, UserError> {
    let pool = pool_handler(pool)?;
    Product::destroy(&id.into_inner(), &pool)
        .map(|_| HttpResponse::NoContent().json(()))
        .map_err(|err| UserError::InternalServerError(err.to_string()))
}

// Update a product by id
#[put("/{id}")]
async fn update(
    id: web::Path<i32>,
    new_product: web::Json<NewProduct>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, UserError> {
    let pool = pool_handler(pool)?;
    Product::update(&id.into_inner(), &new_product, &pool)
        .map(|product| HttpResponse::Ok().json(product))
        .map_err(|err| UserError::InternalServerError(err.to_string()))
}
