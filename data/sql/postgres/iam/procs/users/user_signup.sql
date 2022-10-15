create or replace procedure user_signup(
    p_user_id iam.users.id%type,
    p_email iam.users.email%type,
    p_password iam.users.pw%type
)
language plpgsql
as $$
begin
    insert into iam.users (
        id,
        active,
        email,
        pw
    ) values (
        p_user_id,
        true,
        p_email,
        p_password
    );
end
$$;