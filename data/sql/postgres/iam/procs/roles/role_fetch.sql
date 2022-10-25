create or replace function role_fetch(
    p_client_id iam.roles.client_id%type,
    p_filter varchar(100),
    p_items int default 10,
    p_page int default 0
)
returns table (
    id iam.roles.id%type,
    active iam.roles.active%type,
    name iam.roles.name%type,
    description iam.roles.description%type
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
            a.description
        from iam.roles a
        where
            a.client_id = p_client_id
        order by
            a.name
        limit p_items
        offset p_items * p_page
        ;
    else
        return query
        select
            a.id,
            a.active,
            a.name,
            a.description
        from iam.roles a
        where
            a.client_id = p_client_id
            and concat(
                a.name,
                a.description
            ) like p_filter
        order by
            a.name
        limit p_items
        offset p_items * p_page
        ;
    end if;
end
$$;