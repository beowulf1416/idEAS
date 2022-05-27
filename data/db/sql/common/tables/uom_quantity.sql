create table uom_quantity (
    id int not null,
    name varchar(20) not null,
    symbol varchar(10),

    constraint pk_uom_quantity
        primary key (id),
    constraint u_uom_quantity_1
        unique (name)
);


insert into common.uom_quantity (id, name, symbol) values 
(1, 'piece', 'pc'),
(2, 'box')
;