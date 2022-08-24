create or replace function domain_get_by_slug (
    p_slug domain.domains.slug%type
)
returns table (
    id domain.domains.id%type,
    active domain.domains.active%type,
    name domain.domains.name%type,
    slug domain.domains.slug%type
)
language plpgsql
as $$
begin
    return query
    select
        a.id,
        a.active,
        a.name,
        a.slug
    from domain.domains a
    where
        a.slug = p_slug
    ;
end
$$;