create table domains (
    id uuid not null,
    active boolean not null default false,
    created timestamp without time zone not null default(now() at time zone 'utc'),

    name text not null,
    -- slug varchar(100) not null,
    slug common.slug_text not null,

    constraint pk_domains
        primary key (id),
    constraint u_domains_1
        unique (name),
    constraint u_domains_2
        unique (slug)
);