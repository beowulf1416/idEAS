create or replace procedure assign_user_to_role (
    p_user_id iam.user_roles.user_id%type,
    p_role_id iam.user_roles.role_id%type
)
language plpgsql
as $$
begin
    insert into iam.user_roles (
        user_id,
        role_id
    ) values (
        p_user_id,
        p_role_id
    );
end
$$;