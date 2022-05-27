create table dimensions (
    id smallint not null,
    name varchar(20) not null,

    constraint pk_dimensions
        primary key (id),
    constraint u_dimensions_1
        unique (name)
);


insert into dimensions (id, name) values
(1, 'quantity/count'),
(2, 'length'),
(3, 'area'),
(4, 'volume'),
(5, 'mass')
;