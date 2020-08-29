CREATE TABLE "staff_hours" (
  staff_hours_id SERIAL PRIMARY KEY,
  staff_id INT NOT NULL,
  day_of_week INT NOT NULL,
  start_time TIME,
  end_time TIME,
  CONSTRAINT fk_staff
    FOREIGN KEY(staff_id) 
    REFERENCES staff(staff_id)
);