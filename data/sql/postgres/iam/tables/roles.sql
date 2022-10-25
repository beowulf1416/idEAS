create table roles (
    id uuid not null,
    client_id uuid not null,
    active boolean not null default false,
    created timestamp without time zone not null default(now() at time zone 'utc'),
    
    name varchar(100) not null,
    description text not null,

    constraint pk_roles
        primary key (id),
    constraint u_roles_1
        unique (client_id, name),
    constraint fk_roles_1
        foreign key (client_id)
        references client.clients (id)
        on delete restrict
);

comment on table roles is 'table of roles';