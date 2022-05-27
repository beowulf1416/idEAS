create table uom_volume (
    id int not null,
    name varchar(20) not null,
    symbol varchar(10),
    conversion decimal(16,8) not null default 1,

    constraint pk_uom_volume
        primary key (id),
    constraint u_uom_volume_1
        unique (name)
);


insert into uom_volume values (id, name, symbol, conversion) values 
(1, 'cubic meter', 'm³', 1),
(2, 'liter', 'l', 0.001),
(3, 'quart', 'qt', 0.000946353),
(4, 'pint', 'p', 0.000473176)
;