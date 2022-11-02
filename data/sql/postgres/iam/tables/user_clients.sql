create table user_clients (
    active boolean not null default false,
    created timestamp without time zone not null default(now() at time zone 'utc'),

    user_id uuid not null,
    client_id uuid not null,

    constraint pk_user_clients
        primary key (user_id, client_id),
    
    -- constraint u_user_clients_1
    --     unique (user_id, client_id),
    
    constraint fk_user_clients_1
        foreign key (user_id)
        references iam.users (id)
        on delete restrict,
    constraint fk_user_clients_2
        foreign key (client_id)
        references client.clients (id)
        on delete restrict
);

comment on table user_clients is 'table mapping users to clients';