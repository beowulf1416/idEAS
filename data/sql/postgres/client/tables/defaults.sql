create table defaults (
    client_id uuid not null,

    currency_id char(3) not null,

    constraint pk_defaults
        primary key (id),

    constraint fk_defaults_1
        foreign key (client_id)
        references clients (id)
        on delete restrict
);