create or replace procedure user_register(
    p_id iam.user_registration.id%type,
    p_email iam.user_registration.email%type
)
language plpgsql
as $$
begin
    insert into iam.user_registration (
        id,
        email
    ) values (
        p_id,
        p_email
    );
end
$$;