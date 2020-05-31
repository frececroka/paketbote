Create Table job
(
    id     Serial Primary Key,
    tag    Varchar(255) Not Null,
    spec   Jsonb        Not Null,
    worker Varchar(255)
);
