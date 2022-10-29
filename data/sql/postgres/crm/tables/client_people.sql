create table client_people(
    client_id uuid not null,
    people_id uuid not null,

    constraint pk_client_people
        primary key (client_id, people_id),

    constraint fk_client_people_1
        foreign key (client_id)
        references client.clients (id)
        on delete restrict,
    constraint fk_client_people_2
        foreign key (people_id)
        references crm.people (id)
        on delete restrict
);

comment on table client_people is 'table mapping people to clients';