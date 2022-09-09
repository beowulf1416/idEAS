create table currencies (
    currency varchar(10) not null,
    unit_text varchar(100) not null,
    symbol varchar(10),

    constraint pk_currencies
        primary key (currency),

    constraint u_currencies_1
        unique (unit_text)
);