create table user_tenants (
    active boolean not null default false,
    created timestamp without time zone not null default(now() at time zone 'utc'),
    user_id uuid not null,
    domain_id uuid not null,

    constraint pk_user_domains
        primary key (user_id, domain_id),
    constraint u_user_domains_1
        unique (user_id),
    constraint fk_user_domains_1
        foreign key (user_id)
        references iam.users (id)
        on delete restrict,
    constraint fk_user_domains_2
        foreign key (domain_id)
        references domain.domains (id)
        on delete restrict
);

comment on table user_tenants is 'table mapping users to domains';