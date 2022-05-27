create table uom_mass (
    id int not null,
    name varchar(20) not null,
    symbol varchar(10),
    conversion decimal(7,4) not null default 1,

    constraint pk_uom_mass
        primary key (id),
    constraint u_uom_mass_1
        unique (name)
);


insert into uom_mass (id, name, symbol, conversion) values 
(1, 'kilogram', 'kg', 1),
(2, 'gram', 'g', 0.001),
(3, 'pound', 'lb', 0.453592)
;