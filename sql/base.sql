CREATE TABLE Ventas (
                        id_venta     SERIAL PRIMARY KEY,
                        id_producto  INT NOT NULL REFERENCES Productos(id_producto),
                        cantidad     INT NOT NULL,
                        total        NUMERIC(10, 2) NOT NULL,
                        fecha_venta  TIMESTAMP NOT NULL DEFAULT NOW()
);