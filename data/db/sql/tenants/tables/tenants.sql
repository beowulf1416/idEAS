create table tenants (
    id uuid not null,
    active boolean not null default false,
    created timestamp without time zone not null default(now() at time zone 'utc'),
    name varchar(100) not null,
    constraint pk_tenants 
        primary key (id),
    constraint u_tenants_1 
        unique (name)
);

comment on table tenants is 'table of tenants';