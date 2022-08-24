create or replace function user_get_by_email(
    p_email iam.users.email%type
)
returns table (
    id iam.users.id%type,
    active iam.users.active%type,
    email iam.users.email%type,
    given_name iam.users.given_name%type,
    family_name iam.users.family_name%type,
    prefix iam.users.honorific_prefix%type,
    suffix iam.users.honorific_suffix%type
)
language plpgsql
as $$
begin
    return query
    select
        a.id,
        a.active,
        a.email,
        a.given_name,
        a.family_name,
        a.honorific_prefix,
        a.honorific_suffix
    from iam.users a
    where
        a.email = p_email
    ;
end
$$;