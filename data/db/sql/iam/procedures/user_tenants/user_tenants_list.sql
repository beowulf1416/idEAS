create or replace function user_tenants_list(
    p_tenant_id iam.user_tenants.tenant_id%type,
    p_filter varchar(100) default '',
    p_items int default null,
    p_page int default null
)
returns table (
    user_id iam.users.id%type,
    active iam.users.active%type,
    email iam.users.email%type
)
language plpgsql
as $$
begin
    if p_filter = '' then
        begin
            return query
            select
            from iam.users a
                join iam.user_tenants b
                    on a.id = b.user_id
            where
                b.tenant_id = p_tenant_id
            limit p_items
            offset p_items * p_page
            ;
        end
    else
        begin
            return query
            select
            from iam.users a
                join iam.user_tenants b
                    on a.id = b.user_id
            where
                b.tenant_id = p_tenant_id
                and a.email like p_filter
            limit p_items
            offset p_items * p_page
            ;
        end
    end if;
end
$$;