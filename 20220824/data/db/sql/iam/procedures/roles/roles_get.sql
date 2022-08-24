create or replace function roles_get (
    p_tenant_id iam.roles.tenant_id%type,
    p_filter varchar(100),
    p_items int default null,
    p_page int default null
)
returns table (
    role_id iam.roles.id%type,
    active iam.roles.active%type,
    name iam.roles.name%type,
    description iam.roles.description%type
)
language plpgsql
as $$
begin
    if p_filter = '' then
        begin
            return query
            select
                a.id,
                a.active,
                a.name,
                a.description
            from iam.roles a
            order by
                a.id
            limit p_items
            offset p_items * p_page
            ;
        end
    else
        begin
            return query
            select
                a.id,
                a.active,
                a.name,
                a.description
            from iam.roles a
            where
                concat(
                    a.name,
                    a.description
                ) like p_filter
            order by
                a.id
            limit p_items
            offset p_items * p_page
            ;
        end
    end if;
end
$$;