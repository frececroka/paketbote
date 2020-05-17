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
    owner_id Integer      Not Null References account,
    Unique (owner_id, name)
);

Create Table package
(
    id          Serial Primary Key,
    name        Varchar(255) Not Null,
    version     Varchar(255) Not Null,
    arch        Varchar(255) Not Null,
    size        Integer      Not Null,
    archive     Varchar(255) Not Null,
    signature   Varchar(255) Not Null,
    compression Varchar(255) Not Null,
    created     Timestamp    Not Null Default current_timestamp,
    active      Boolean      Not Null Default False,
    deleted     Boolean      Not Null Default False,
    repo_id     Integer      Not Null References repo,
    Unique (repo_id, name, version)
);

Create Table package_depends
(
    id         Serial Primary Key,
    package_id Integer      Not Null References package,
    depends    Varchar(255) Not Null,
    Unique (package_id, depends)
);

Create Table package_provides
(
    id         Serial Primary Key,
    package_id Integer      Not Null References package,
    provides   Varchar(255) Not Null,
    Unique (package_id, provides)
);

Create Table repo_action
(
    id         Serial Primary Key,
    package_id Integer References package Not Null,
    action     Varchar(255)               Not Null,
    worker     Varchar(255)
);
