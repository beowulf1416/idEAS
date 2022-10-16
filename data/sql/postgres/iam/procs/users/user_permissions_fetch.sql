create or replace function user_permissions_fetch(
    p_user_id iam.users.id%type,
    p_client_id client.clients.id%type
)
returns table(
    name iam.permissions.name%type
)
language plpgsql
as $$
begin
    return query
    select
        distinct
            a.name
    from iam.permissions a
        join iam.role_permissions b
            on a.id = b.permission_id
        join iam.user_roles c
            on b.role_id = c.role_id
    where
        c.user_id = p_user_id
        -- filter out inactive permissions
        and a.active = true
        -- filter out inactive role permissions
        and b.active = true
        -- filter out inactive user roles
        and c.active = true
    ;
end
$$;