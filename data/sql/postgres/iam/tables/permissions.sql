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
(6, true, 'domain.add', 'add domain'),
(7, true, 'domain.active', 'set domain active status'),
(8, true, 'domain.view', 'view tenant information'),
(9, true, 'domain.users.view', 'view domain users'),
(10, true, 'domain.users.add', 'add user to domain'),
(11, true, 'domain.admin', 'administer domain'),
(12, true, 'domain.admin.list', 'view list of domain'),
(13, true, 'domain.admin.view', 'view tenant information')
;