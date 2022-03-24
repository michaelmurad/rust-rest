DROP TABLE IF EXISTS  users;
CREATE TABLE users (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid (),
    email varchar(255) NOT NULL UNIQUE,
    firstName varchar(50) NOT NULL,
    lastName varchar(50) NOT NULL,
    username varchar (50) NOT NULL UNIQUE
);