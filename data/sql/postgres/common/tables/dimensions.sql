create table dimensions (
    id smallint not null,
    name varchar(100) not null,

    constraint pk_dimensions
        primary key (id)
);


insert into dimensions (id, name) values 
(1, 'quantity'),
(2, 'length'),
(3, 'area'),
(4, 'volume'),
(5, 'time')
;