create or replace function user_register_get(
    p_id iam.user_registration.id%type
)
returns table (
    id iam.user_registration.id%type,
    email iam.user_registration.email%type
)
language plpgsql
as $$
begin
    return query
    select
        ur.id,
        ur.email
    from iam.user_registration ur
    where
        ur.id = p_id
    ;
end
$$;