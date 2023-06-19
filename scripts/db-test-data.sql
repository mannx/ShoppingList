-- Generate some test data for the db for dev purposes

BEGIN TRANSACTION;

INSERT INTO Locations (name)
    VALUES ('Store #1'), ('Store #2');


END TRANSACTION;
