create table tasks (
    id uuid not null,
    client_id uuid not null,
    active boolean not null default false,
    created timestamp without time zone not null default(now() at time zone 'utc'),

    name varchar(200) not null,
    description text,

    pay_rate_type_id smallint not null,

    constraint pk_tasks
        primary key (id),

    constraint fk_tasks_1
        foreign key (client_id)
        references client.clients (id)
        on delete restrict,
    constraint fk_tasks_2
        foreign key hr.pay_rate_types (id)
        on delete restrict  
);