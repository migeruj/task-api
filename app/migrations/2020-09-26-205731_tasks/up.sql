-- Your SQL goes here
CREATE TABLE tasks (
  id INTEGER PRIMARY KEY AUTO_INCREMENT,
  name VARCHAR(45) NOT NULL,
  done BOOLEAN NOT NULL DEFAULT FALSE,
  created_at TIMESTAMP NOT NULL DEFAULT now(),
  update_at TIMESTAMP NOT NULL DEFAULT now() ON UPDATE now()
);