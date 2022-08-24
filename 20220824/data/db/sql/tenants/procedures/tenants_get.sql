create or replace function tenants_get (
    p_filter tenants.tenants.name%type default '',
    p_items int default null,
    p_page int default null
)
returns table (
    id tenants.tenants.id%type,
    active tenants.tenants.active%type,
    name tenants.tenants.name%type
)
language plpgsql
as $$
begin
    if p_filter = '' then
        return query
        select
            a.id,
            a.active,
            a.name
        from tenants.tenants a
        order by
            a.id
        limit p_items
        offset p_items * p_page
        ;
    else
        return query
        select
            a.id,
            a.active,
            a.name
        from tenants.tenants a
        where
            a.name like p_filter
        order by
            a.id
        limit p_items
        offset p_items * p_page
        ;
    end if;
end
$$;