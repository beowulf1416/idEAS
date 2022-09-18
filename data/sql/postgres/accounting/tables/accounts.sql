create table accounts (
    id uuid not null,
    client_id uuid not null,
    active boolean not null default false,
    created timestamp without time zone not null default(now() at time zone 'utc'),

    type_id int not null,
    name varchar(100) not null,
    description text,

    constraint pk_accounts
        primary key (id),
    
    constraint u_accounts_1
        unique (client_id, name),

    constraint fk_accounts_1
        foreign key (client_id)
        references client.clients (id)
        on delete restrict,
    constraint fk_accounts_2
        foreign key (type_id)
        references accounting.account_types (id)
        on delete restrict
);


create table accounts_scd (
    account_id uuid not null,
    active boolean not null,
    created timestamp without time zone not null default(now() at time zone 'utc'),

    type_id int not null,
    name varchar(100) not null,
    description text,

    constraint fk_accounts_scd_1
        foreign key (account_id)
        references accounting.accounts (id)
        on delete restrict,
    constraint fk_accounts_scd_2
        foreign key (type_id)
        references accounting.account_types (id)
        on delete restrict
);