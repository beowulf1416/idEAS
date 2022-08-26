create table defaults (
    domain_id uuid not null,
    created timestamp without time zone not null default(now() at time zone 'utc'),

    currency_id char(3) not null,

    constraint pk_defaults
        primary key (domain_id),

    constraint fk_defaults_1
        foreign key (domain_id)
        references domains (id)
        on delete restrict,
    constraint fk_defauls_2
        foreign key (currency_id)
        references common.currencies (id)
        on delete restrict
);