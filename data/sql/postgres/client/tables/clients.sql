create table clients (
    id uuid not null,
    active boolean not null,
    created timestamp without time zone not null default(now() at time zone 'utc'),

    name varchar(300) not null,
    description text,

    address text,
    country_id int not null,

    url varchar(300),

    constraint pk_clients
        primary key (id),
    
    constraint u_clients_1
        unique (name),

    constraint fk_clients_1
        foreign key (country_id)
        references common.countries (iso_3166_1_numeric)
        on delete restrict
);


create table clients_scd (
    client_id uuid not null,
    active boolean not null,
    created timestamp without time zone not null default(now() at time zone 'utc'),

    name varchar(300) not null,
    description text,

    address text,
    country_id int not null,

    url varchar(300),

    constraint fk_clients_scd_1
        foreign key (client_id)
        references clients (id)
        on delete restrict
);