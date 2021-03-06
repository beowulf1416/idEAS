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
(1, true, 'user.current', 'get current user information'),
(2, true, 'user.profile', 'view user profile'),
(3, true, 'dashboard.view', 'view dashboard'),
(4, true, 'site.admin.dashboard.view', 'view site admin dashboard'),
(5, true, 'site.admin.tenants.view', 'view site admin tenants'),
(6, true, 'tenant.add', 'add tenant'),
(7, true, 'tenant.active', 'set tenant active status'),
(8, true, 'tenant.view', 'view tenant information'),
(9, true, 'tenant.users.view', 'view tenant users'),
(10, true, 'tenant.users.add', 'add user to tenant'),
(11, true, 'tenants.admin', 'administer tenants'),
(12, true, 'tenants.admin.list', 'view list of tenants'),
(13, true, 'tenants.admin.view', 'view tenant information')
;