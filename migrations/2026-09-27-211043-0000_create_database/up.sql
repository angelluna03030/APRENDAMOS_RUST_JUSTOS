-- Your SQL goes here
CREATE TABLE clientes (
    id SERIAL PRIMARY KEY,
    nombre VARCHAR(100) NOT NULL,
    telefono VARCHAR(20)
);

CREATE TABLE carros (
    id SERIAL PRIMARY KEY,
    cliente_id INTEGER NOT NULL,
    placa VARCHAR(10) UNIQUE NOT NULL,
    marca VARCHAR(50) NOT NULL,
    modelo VARCHAR(50),

    CONSTRAINT fk_carro_cliente
        FOREIGN KEY (cliente_id)
        REFERENCES clientes(id)
);

CREATE TABLE reparaciones (
    id SERIAL PRIMARY KEY,
    carro_id INTEGER NOT NULL,
    descripcion TEXT NOT NULL,
    estado VARCHAR(30) DEFAULT 'pendiente',
    precio NUMERIC(10,2) DEFAULT 0,

    CONSTRAINT fk_reparacion_carro
        FOREIGN KEY (carro_id)
        REFERENCES carros(id)
);