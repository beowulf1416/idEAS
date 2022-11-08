create or replace function role_permissions_not_assigned(
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
    where
        a.id not in (
            select
                b.permission_id
            from iam.role_permissions b
            where
                b.role_id = p_role_id
        )
    ;
end
$$;