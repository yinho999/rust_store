-- Your SQL goes here

-- Create user table 
CREATE TABLE users
(
    id SERIAL PRIMARY KEY,
    email varchar(100) NOT NULL,
    company VARCHAR(100) NOT NULL,
    password varchar(64) NOT NULL,
    created_at timestamp NOT NULL
);

-- Create Index on users table
CREATE INDEX users_email_company_idx ON users (email, company);