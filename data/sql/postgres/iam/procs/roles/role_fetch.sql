create or replace function role_fetch(
    p_domain_id iam.roles.client_id%type,
    p_filter varchar(100),
    p_items int default 10,
    p_page int default 0
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
    if p_filter = '' then
        return query
        select
            a.id,
            a.client_id,
            a.active,
            a.name,
            a.slug
        from iam.roles a
        order by
            a.name
        limit p_items
        offset p_items * p_page
        ;
    else
        return query
        select
            a.id,
            a.client_id,
            a.active,
            a.name,
            a.slug
        from iam.roles a
        where
            concat(
                a.name,
                a.slug
            ) like p_filter
        order by
            a.name
        limit p_items
        offset p_items * p_page
        ;
    end if;
end
$$;