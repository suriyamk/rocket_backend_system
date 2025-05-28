CREATE TABLE user
(
    userId INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    userName TEXT NOT NULL,
    companyId INTEGER NOT NULL,
    isDeleted BOOLEAN NOT NULL,
    createdDate DATE NOT NULL,
    modifiedDate DATE NOT NULL
)