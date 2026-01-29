-- Add migration script here
-- migrations/xxxx_init_schema.sql

CREATE TYPE sample_category AS ENUM ('BLOOD', 'URINE', 'STOOL', 'MUCOSA', 'SERUM', 'OTHER');
CREATE TYPE user_role AS ENUM ('admin', 'operator');

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    role user_role DEFAULT 'operator'
);

CREATE TABLE patients (
    id SERIAL PRIMARY KEY,
    identifier VARCHAR(20) UNIQUE NOT NULL,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    birth_date DATE NOT NULL,
    gender CHAR(1) CHECK (gender IN ('M', 'F', 'O'))
);

CREATE TABLE test_types (
    id SERIAL PRIMARY KEY,
    name VARCHAR(150) NOT NULL,
    cost DECIMAL(12, 2) NOT NULL,
    sample_type sample_category NOT NULL
);

CREATE TABLE test_parameters (
    id SERIAL PRIMARY KEY,
    test_type_id INTEGER REFERENCES test_types(id),
    name VARCHAR(100) NOT NULL,
    unit VARCHAR(20),
    reference_range TEXT
);

CREATE TABLE orders (
    id SERIAL PRIMARY KEY,
    patient_id INTEGER REFERENCES patients(id),
    total_amount DECIMAL(12, 2) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE results (
    id SERIAL PRIMARY KEY,
    order_id INTEGER REFERENCES orders(id),
    parameter_id INTEGER REFERENCES test_parameters(id),
    test_value TEXT,
    is_abnormal BOOLEAN DEFAULT FALSE
);