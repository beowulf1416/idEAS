create table currencies (
    country_name varchar(100) not null,
    currency varchar(100) not null,
    currency_alpha_3 char(3),
    currency_numeric int,
    minor_unit varchar(50),
    fund varchar(50)
);

create index if not exists idx_currencies_1
on currencies (currency_numeric);