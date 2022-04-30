create or replace procedure assign_permission_to_role (
    p_role_id iam.role_permissions.role_id%type,
    p_permission_id iam.role_permissions.permission_id%type
)
language plpgsql
as $$
begin
    insert into iam.role_permissions (
        role_id,
        permission_id
    ) values (
        p_role_id,
        p_permission_id
    );
end
$$;