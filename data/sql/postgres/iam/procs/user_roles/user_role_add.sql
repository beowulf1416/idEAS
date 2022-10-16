create or replace procedure user_role_add(
    p_user_id iam.users.id%type,
    p_role_id iam.roles.id%type
)
language plpgsql
as $$
begin
    insert into iam.user_roles (
        active, 
        user_id, 
        role_id
    ) values (
        true,
        p_user_id,
        p_role_id
    );
end
$$;