CREATE TABLE service (
    service_id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description VARCHAR,
    is_active INT NOT NULL DEFAULT(1),
    category VARCHAR
);

CREATE TABLE service_variant (
    service_variant_id SERIAL PRIMARY KEY,
    service_id INT NOT NULL,
    price FLOAT NOT NULL,
    duration TIME,
    CONSTRAINT fk_service
        FOREIGN KEY(service_id) 
        REFERENCES service(service_id)
);

CREATE TABLE block_extra_time (
    block_extra_time_it SERIAL PRIMARY KEY,
    service_id INT NOT NULL,
    before_time TIME,
    after_time TIME,
    CONSTRAINT fk_service
        FOREIGN KEY(service_id) 
        REFERENCES service(service_id)
);