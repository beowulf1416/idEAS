create table currencies (
    currency_id char(3) not null,
    name varchar(100) not null,
    symbol varchar(10),

    constraint pk_currencies
        primary key (currency_id),
    constraint u_currencies_1
        unique (name),
    constraint u_currencies_2
        unique (symbol)
);

