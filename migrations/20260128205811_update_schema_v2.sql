-- Add migration script here
-- 1. Actualizar test_types para incluir descripción
ALTER TABLE test_types ADD COLUMN description TEXT;

-- 2. Actualizar test_parameters para incluir el tipo de dato
ALTER TABLE test_parameters ADD COLUMN data_type VARCHAR(20) DEFAULT 'numeric';

-- 3. Mejorar la tabla de órdenes para contabilidad y usuarios
ALTER TABLE orders ADD COLUMN created_by INTEGER REFERENCES users(id);
ALTER TABLE orders ADD COLUMN payment_status VARCHAR(20) DEFAULT 'PAID';

-- 4. Mejorar la tabla de resultados para trazabilidad
ALTER TABLE results ADD COLUMN technician_id INTEGER REFERENCES users(id);
ALTER TABLE results ADD COLUMN validated_at TIMESTAMP;