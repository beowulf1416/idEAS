create or replace function role_get_by_slug(
    p_slug iam.roles.slug%type
)
returns table (
    id iam.roles.id%type,
    client_id iam.roles.client_id%type,
    active iam.roles.active%type,
    name iam.roles.name%type,
    slug iam.roles.slug%type
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
        a.slug
    from iam.roles a
    where
        a.slug = p_slug
    ;
end
$$;