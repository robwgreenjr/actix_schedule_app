create table staff_service (
    staff_service_id SERIAL PRIMARY KEY,
    staff_id INT NOT NULL,
    service_variant_id INT NOT NULL,
    is_active INT DEFAULT(0),
    CONSTRAINT fk_service_variant
        FOREIGN KEY(service_variant_id) 
        REFERENCES service_variant(service_variant_id),
    CONSTRAINT fk_staff
        FOREIGN KEY(staff_id) 
        REFERENCES staff(staff_id)
);