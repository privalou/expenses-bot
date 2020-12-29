CREATE TABLE users
(
    id       varchar(20) PRIMARY KEY NOT NULL,
    currency varchar(32)
);

CREATE TABLE dialogs
(
    user_id varchar(20) PRIMARY KEY NOT NULL REFERENCES users (id) ON DELETE CASCADE ON UPDATE CASCADE,
    command varchar(32)             NOT NULL,
    step    varchar(32)
);

create table history
(
    id       serial PRIMARY KEY,
    user_id  varchar(20)              NOT NULL REFERENCES users (id) ON DELETE CASCADE ON UPDATE CASCADE,
    amount   float4                   NOT NULL,
    category varchar(32),
    created  TIMESTAMP WITH TIME ZONE NOT NULL
);
