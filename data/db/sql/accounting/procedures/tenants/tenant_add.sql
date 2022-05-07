create or replace procedure tenant_add (
    p_tenant_id accounting.tenants.tenant_id%type,
    p_currency_alpha_3 accounting.tenants.currency_alpha_3%type
)
language plpgsql
as $$
begin
    insert into accounting.tenants (
        tenant_id,
        currency_alpha_3
    ) values (
        p_tenant_id,
        p_currency_alpha_3
    );
end
$$;