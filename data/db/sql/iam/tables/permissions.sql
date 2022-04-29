create table permissions (
    id bigint not null,
    active boolean not null default false,
    created timestamp without time zone not null default(now() at time zone 'utc'),
    name varchar(100) not null,
    description text not null,

    constraint pk_permissions
        primary key (id),
    constraint u_permissions_1
        unique (name)
);

comment on table permissions is 'table of permissions';



-- create permissions
insert into iam.permissions (id, active, name, description) values
(1, true, 'tenant.create', 'add tenant'), 
(2, true, 'tenant.view', 'view tenant information'),
(3, true, 'tenant.users.view', 'view tenant users'),
(4, true, 'tenant.users.add', 'add user to tenant'),
(5, true, 'tenant.view', 'view tenant information')
;