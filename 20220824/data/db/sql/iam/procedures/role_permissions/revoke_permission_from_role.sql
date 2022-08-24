create or replace procedure revoke_permission_from_role (
    p_role_id iam.role_permissions.role_id%type,
    p_permission_id iam.role_permissions.permission_id%type
)
language plpgsql
as $$
begin
    delete from iam.role_permissions
    where
        role_id = p_role_id
        and permission_id = p_permission_id
    ;
end
$$;