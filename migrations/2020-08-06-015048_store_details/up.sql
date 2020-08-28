-- Your SQL goes here
CREATE TABLE store (
  store_id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  store_address_id INT NOT NULL
);

CREATE TABLE store_hours (
  store_hours_id SERIAL PRIMARY KEY,
  store_id INT NOT NULL,
  day_of_week INT NOT NULL,
  start_time TIME,
  end_time TIME,
  CONSTRAINT fk_store
    FOREIGN KEY(store_id) 
    REFERENCES store(store_id)
);

CREATE TABLE store_address (
  store_address_id SERIAL PRIMARY KEY,
  store_id INT NOT NULL,
  street_address VARCHAR NOT NULL,
  city VARCHAR NOT NULL,
  state VARCHAR NOT NULL,
  zip INT NOT NULL,
  phone VARCHAR NOT NULL,
  email VARCHAR NOT NULL,
  CONSTRAINT fk_store
    FOREIGN KEY(store_id) 
    REFERENCES store(store_id)
);