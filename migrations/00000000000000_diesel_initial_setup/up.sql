Create Table account
(
    id              Serial Primary Key,
    name            Varchar(255) Not Null Unique,
    salt            Varchar(255) Not Null,
    hashed_password Varchar(255) Not Null
);

Create Table token
(
    id         Serial Primary Key,
    name       Varchar(255) Not Null,
    the_token  Varchar(255) Not Null,
    account_id Integer      Not Null References account
);

Create Table repo
(
    id       Serial Primary Key,
    name     Varchar(255) Not Null,
    owner_id Integer      Not Null References account
);

Create Table package
(
    id        Serial Primary Key,
    name      Varchar(255) Not Null,
    version   Varchar(255) Not Null,
    arch      Varchar(255) Not Null,
    size      Integer      Not Null,
    archive   Varchar(255) Not Null,
    signature Varchar(255) Not Null,
    repo_id   Integer      Not Null References repo,
    Unique (repo_id, name, version)
);

Create Table repo_add
(
    id         Serial Primary Key,
    package_id Integer References package Not Null,
    worker     Varchar(255)
);
