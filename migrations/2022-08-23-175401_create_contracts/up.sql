CREATE TABLE contracts (
    id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    the_owner VARCHAR(255) NOT NULL,
    the_target VARCHAR(255) NOT NULL,
    bounty INT NOT NULL,
    finished BOOLEAN DEFAULT FALSE
);