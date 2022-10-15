create or replace procedure user_role_set_active(
    p_user_id iam.users.id%type,
    p_role_id iam.roles.id%type,
    p_active iam.user_roles.active%type
)
language plpgsql
as $$
begin
    update iam.user_roles set
        active = p_active
    where
        user_id = p_user_id
        and role_id = p_role_id
    ;
end
$$;