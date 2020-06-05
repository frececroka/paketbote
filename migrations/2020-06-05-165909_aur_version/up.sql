Create Table aur_version
(
    id      Serial Primary Key,
    package Varchar(255) Unique Not Null,
    version Varchar(255)        Not Null
);
