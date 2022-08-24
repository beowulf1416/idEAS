create table uom_length (
    id int not null,
    name varchar(20) not null,
    symbol varchar(10),
    conversion decimal(7,4) not null default 1,

    constraint pk_uom_length
        primary key (id),
    constraint u_uom_length_1
        unique (name)
);


insert into common.uom_length (id, name, symbol, conversion) values
(1, 'meter', 'm', 1),
(2, 'kilometer', 'km', 1000),
(3, 'centimeter', 'cm', 0.01),
(4, 'feet', 'ft', 0.3048),
(5, 'inch', 'in', 0.0254)
;