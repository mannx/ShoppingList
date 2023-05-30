-- Add up migration script here

CREATE TABLE IF NOT EXISTS Locations (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS ShoppingList (
    id SERIAL PRIMARY KEY,
    item VARCHAR(255) NOT NULL,
    location int NOT NULL,
    CONSTRAINT fk_location
        FOREIGN KEY(location)
        REFERENCES Locations(id)
);
