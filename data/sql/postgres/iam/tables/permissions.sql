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



/*
-- create permissions
insert into iam.permissions (id, active, name, description) values
(1, true, 'user.current', 'get current user information'),
(2, true, 'user.profile', 'view user profile'),
(3, true, 'dashboard.view', 'view dashboard'),
(4, true, 'site.admin.dashboard.view', 'view site admin dashboard'),
(5, true, 'site.admin.clients.view', 'view site admin clients'),
(6, true, 'client.add', 'add client'),
(7, true, 'client.update', 'update client'),
(8, true, 'client.update.any', 'update any client'),
(9, true, 'client.active', 'set client active status'),
(10, true, 'client.view', 'view client information'),
(11, true, 'client.view.any', 'view any client information'),
(12, true, 'client.users.view', 'view client users'),
(13, true, 'client.users.add', 'add user to client'),
(14, true, 'client.admin', 'administer client'),
(15, true, 'client.admin.list', 'view list of client'),
(16, true, 'client.admin.view', 'view tenant information')
;
*/



\copy iam.permissions (id, active, name, description) from '/docker-entrypoint-initdb.d/permissions.csv' with delimiter ',' csv header quote '"';