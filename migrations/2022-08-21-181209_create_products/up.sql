-- Your SQL goes here
CREATE TABLE products
(
    id serial PRIMARY KEY,
    name varchar(255) NOT NULL,
    stock FLOAT NOT NULL,
    price INTEGER
);