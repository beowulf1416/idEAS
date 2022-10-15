create or replace procedure role_permission_add(
    p_role_id iam.roles.id%type,
    p_permission_id iam.permissions.id%type
)
language plpgsql
as $$
begin
    insert into iam.role_permissions (
        active,
        role_id,
        permission_id
    ) values (
        true,
        p_role_id,
        p_permission_id
    );
end
$$;