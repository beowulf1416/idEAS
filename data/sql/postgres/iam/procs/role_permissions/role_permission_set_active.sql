create or replace procedure role_permission_set_active(
    p_role_id iam.roles.id%type,
    p_permission_id iam.permissions.id%type,
    p_active iam.role_permissions.active%type
)
language plpgsql
as $$
begin
    update iam.role_permissions set
        active = p_active
    where
        role_id = p_role_id
        and permission_id = p_permission_id
    ;
end
$$;