create or replace procedure tenant_add (
    tenant_id tenants.tenants.id%type,
    tenant_name tenants.tenants.name%type
)
language plpgsql
as $$
begin
    insert into tenants.tenants (
        id,
        name
    ) values (
        tenant_id,
        tenant_name
    );
end
$$;