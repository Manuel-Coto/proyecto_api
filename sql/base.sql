<<<<<<< HEAD
CREATE TABLE Ventas (
                        id_venta     SERIAL PRIMARY KEY,
                        id_producto  INT NOT NULL REFERENCES Productos(id_producto),
                        cantidad     INT NOT NULL,
                        total        NUMERIC(10, 2) NOT NULL,
                        fecha_venta  TIMESTAMP NOT NULL DEFAULT NOW()
=======
CREATE TABLE Categorias (
    id_categoria SERIAL PRIMARY KEY,
    nombre VARCHAR(50) NOT NULL,
    descripcion TEXT
);

CREATE TABLE Proveedores (
    id_proveedor SERIAL PRIMARY KEY,
    nombre_empresa VARCHAR(100) NOT NULL,
    telefono VARCHAR(20),
    contacto VARCHAR(100)
);

CREATE TABLE Productos (
    id_producto SERIAL PRIMARY KEY,
    nombre VARCHAR(100) NOT NULL,
    precio DECIMAL(10,2) NOT NULL,
    stock INT DEFAULT 0,
    id_categoria INT REFERENCES Categorias(id_categoria),
    id_proveedor INT REFERENCES Proveedores(id_proveedor)
);

CREATE TABLE Clientes (
    id_cliente SERIAL PRIMARY KEY,
    nombre VARCHAR(50) NOT NULL,
    apellido VARCHAR(50) NOT NULL,
    correo_electronico VARCHAR(100) UNIQUE
);

CREATE TABLE Ventas (
    id_venta SERIAL PRIMARY KEY,
    fecha TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    id_cliente INT REFERENCES Clientes(id_cliente),
    id_producto INT REFERENCES Productos(id_producto),
    cantidad INT NOT NULL
>>>>>>> e2a185f918ea2b821333a73144dd8a263eca4adc
);