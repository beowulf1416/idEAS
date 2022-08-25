create table item_balances (
    id uuid not null,
    domain_id uuid not null,
    active boolean not null default false,
    created timestamp without time zone not null default(now() at time zone 'utc'),

    balance decimal(12, 3) not null default 0,

    constraint pk_item_balances
        primary key (id),

    constraint fk_item_balances_1
        foreign key (domain_id)
        references domain.domains (id)
        on delete restrict
);

create table item_balances_history (
    id uuid not null,
    domain_id uuid not null,
    active boolean not null default false,
    created timestamp without time zone not null default(now() at time zone 'utc'),

    balance decimal(12, 3) not null default 0,

    constraint fk_item_balances_history_1
        foreign key (domain_id)
        references domain.domains (id)
        on delete restrict
);