Create Table missing_dep
(
    id         Serial Primary Key,
    repo_id    Integer      Not Null References repo,
    dependency Varchar(255) Not Null
);
