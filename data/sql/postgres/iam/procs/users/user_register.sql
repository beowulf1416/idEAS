create or replace procedure user_register(
    p_id iam.user_registration.id%type,
    p_token iam.user_registration.token%type,
    p_email iam.user_registration.email%type
)
language plpgsql
as $$
begin
    insert into iam.user_registration (
        id,
        token,
        email
    ) values (
        p_id,
        p_token,
        p_email
    );
end
$$;