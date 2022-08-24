create or replace function domain_fetch (
    p_filter domain.domains.name%type default '',
    p_items int default 10,
    p_page int default 0
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
    if p_filter = '' then
        return query
        select
            a.id,
            a.active,
            a.name,
            a.slug
        from domain.domains a
        order by
            a.name asc
        limit p_items
        offset p_items * p_page
        ;
    else
        return query
        select
            a.id,
            a.active,
            a.name,
            a.slug
        from domain.domains a
        where
            a.name like p_filter
        order by
            a.name asc
        limit p_items
        offset p_items * p_page
        ;
    end if;
end
$$;