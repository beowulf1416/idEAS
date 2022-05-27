create table uom_area (
    id int not null,
    name varchar(20) not null,
    symbol varchar(10),
    conversion decimal(16,8) not null default 1,

    constraint pk_uom_area
        primary key (id),
    constraint u_uom_area_1
        unique (name)
);


insert into uom_area (id, name, symbol, conversion) values
(1, 'square meter', 'm²', 1),
(2, 'square centimeter', 'cm²', 0.0001),
(3, 'square feet', 'ft²', 0.092903),
(4, 'square inch', 'in²', 0.00064516)
;