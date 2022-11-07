create or replace function role_permissions_get(
    p_role_id iam.roles.id%type
)
returns table (
    name iam.permissions.name%type
)
language plpgsql
as $$
begin
    return query
    select
        a.name
    from iam.permissions a
        join iam.role_permissions b
            on a.id = b.permission_id
    where
        b.role_id = p_role_id
    ;
end
$$;