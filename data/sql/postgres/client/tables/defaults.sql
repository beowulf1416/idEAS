create table defaults (
    client_id uuid not null,

    currency_id varchar(10) not null,

    constraint pk_defaults
        primary key (client_id),

    constraint fk_defaults_1
        foreign key (client_id)
        references client.clients (id)
        on delete restrict,
    constraint fk_defaults_2
        foreign key (currency_id)
        references common.currencies (currency)
        on delete restrict
);