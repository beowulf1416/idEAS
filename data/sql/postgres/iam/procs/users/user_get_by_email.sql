create or replace function user_get_by_email(
    p_email iam.users.email%type
)
returns table (
    id iam.users.id%type,
    active iam.users.active%type,
    email iam.users.email%type
)
language plpgsql
as $$
begin
    return query
    select
        a.id,
        a.active,
        a.email
    from iam.users a
    where
        a.email = p_email
    ;
end
$$;