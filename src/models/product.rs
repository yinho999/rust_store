use crate::db_connection::establish_connection;
use crate::diesel::ExpressionMethods;
use crate::schema::products::dsl::*;
use diesel::PgConnection;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

use serde::{Deserialize, Serialize};

// use product table in schema file
use crate::schema::products;

// Create a struct to represent a product.
#[derive(Queryable, Serialize, Deserialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub stock: f64,
    pub price: Option<i32>,
}

impl Product {
    pub fn find(
        search_id: &i32,
        connection: &PgConnection,
    ) -> Result<Product, diesel::result::Error> {
        let product = products.find(search_id).first(connection)?;
        Ok(product)
    }

    // Delete a product by id
    pub fn destroy(
        search_id: &i32,
        connection: &PgConnection,
    ) -> Result<(), diesel::result::Error> {
        diesel::delete(products.filter(id.eq(search_id))).execute(connection)?;
        Ok(())
    }

    // Update a product by id
    pub fn update(
        search_id: &i32,
        new_product: &NewProduct,
        connection: &PgConnection,
    ) -> Result<Product, diesel::result::Error> {
        let updated_product = diesel::update(products.find(search_id))
            .set(new_product)
            .get_result::<Product>(connection)?;
        Ok(updated_product)
    }
}

/// Create Product
// Create a new product.
#[derive(Insertable, Deserialize, AsChangeset)]
#[table_name = "products"]
pub struct NewProduct {
    pub name: Option<String>,
    pub stock: Option<f64>,
    pub price: Option<i32>,
}

impl NewProduct {
    pub fn create(&self, connection: &PgConnection) -> Result<Product, diesel::result::Error> {
        // Insert the new product into the database.
        diesel::insert_into(products)
            .values(self)
            .get_result(connection)
    }
}

/// Get Product
// Get a list of all products.
#[derive(Serialize, Deserialize)]
pub struct ProductsList(pub Vec<Product>);

impl ProductsList {
    pub fn list(connection: &PgConnection) -> ProductsList {
        // Get all products from the database.
        let result = products
            .load::<Product>(connection)
            .expect("Error loading products");

        // Return the list of products.
        ProductsList(result)
    }
}
