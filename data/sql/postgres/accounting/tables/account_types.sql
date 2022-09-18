create table account_types (
    id int not null,
    name varchar(100) not null,

    constraint pk_account_types
        primary key (id)
);