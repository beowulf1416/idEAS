create or replace function role_get(
    p_id iam.roles.id%type
)
returns table (
    id iam.roles.id%type,
    client_id iam.roles.client_id%type,
    active iam.roles.active%type,
    name iam.roles.name%type,
    description iam.roles.description%type
)
language plpgsql
as $$
begin
    return query
    select
        a.id,
        a.client_id,
        a.active,
        a.name,
        a.description
    from iam.roles a
    where
        a.id = p_id
    ;
end
$$;