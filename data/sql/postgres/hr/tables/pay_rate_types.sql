create table pay_rate_types (
    id smallint not null,
    name varchar(100) not null,
    description text,

    constraint pk_pay_rate_types
        primary key (id)
);


insert into pay_rate_types (id, name, description) values 
(1, 'hourly', 'hourly rate'),
(2, 'per-piece', 'per-piece rate'),
(3, 'per-project', 'per-project rate')
;