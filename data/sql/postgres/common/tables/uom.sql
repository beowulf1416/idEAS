create table uom (
    id bigint not null,
    dimension_id smallint not null,
    name varchar(100) not null,
    description text,
    symbol varchar(10) not null,

    constraint pk_uom
        primary key (id),

    constraint fk_uom_1
        foreign key (dimension_id)
        references dimensions (id)
        on delete restrict
);



insert into uom (id, dimension_id, name, description, symbol) values 
(1, 1, 'piece', 'piece', 'pc')
;