create or replace function permission_fetch (
    p_filter iam.permissions.name%type default '',
    p_items int default 10,
    p_page int default 0
)
returns table (
    id iam.permissions.id%type,
    active iam.permissions.active%type,
    name iam.permissions.name%type,
    description iam.permissions.description%type
)
language plpgsql
as $$
begin
    if p_filter = '' then
        -- select
        --     count(*) into p_total_items
        -- from iam.permissions
        -- ;

        return query
        select
            a.id,
            a.active,
            a.name,
            a.description
        from iam.permissions a
        order by
            a.name asc
        limit p_items
        offset p_items * p_page
        ;
    else
        -- select
        --     count(*) into p_total_items
        -- from iam.permissions
        -- where
        --     name like p_filter
        -- ;

        return query
        select
            a.id,
            a.active,
            a.name,
            a.description
        from iam.permissions a
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