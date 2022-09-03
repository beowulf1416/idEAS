create table organization_tree (
    id uuid not null,
    client_id uuid not null,
    active boolean not null,
    created timestamp without time zone not null default(now() at time zone 'utc'),

    org_id uuid not null,
    parent_org_id uuid not null,

    constraint pk_organization_tree
        primary key (id),

    constraint u_organization_tree
        unique (client_id, org_id, parent_org_id),

    constraint fk_organization_tree_1
        foreign key (client_id)
        references client.clients (id)
        on delete restrict,
    constraint fk_organization_tree_2
        foreign key (org_id)
        references client.organizations (id)
        on delete restrict,
    constraint fk_organization_tree_2
        foreign key (parent_org_id)
        references client.organizations (id)
        on delete restrict
);