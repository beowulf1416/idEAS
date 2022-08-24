create table roles (
    id uuid not null,
    domain_id uuid not null,
    active boolean not null default false,
    created timestamp without time zone not null default(now() at time zone 'utc'),
    
    name varchar(100) not null,
    slug common.slug_text not null,
    description text not null,

    constraint pk_roles
        primary key (id),
    constraint u_roles_1
        unique (domain_id, name),
    constraint u_roles_2
        unique (domain_id, slug),
    constraint fk_roles_1
        foreign key (domain_id)
        references domain.domains (id)
        on delete restrict
        on update restrict
);

comment on table roles is 'table of roles';