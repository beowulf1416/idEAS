create table account_types (
    id int not null,
    active boolean not null default false,
    created timestamp without time zone not null default(now() at time zone 'utc'),
    name varchar(100) not null,

    constraint pk_acct_types
        primary key (id)
);

insert into accounting.account_types (id, active, name) values 
(1, true, 'capital'),
(2, true, 'asset'),
(3, true, 'liability'),
(4, true, 'income'),
(5, true, 'expense')
;