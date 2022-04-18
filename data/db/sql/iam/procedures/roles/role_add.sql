create or replace procedure role_add (
    p_id iam.roles.id%type,
    p_tenant_id iam.roles.tenant_id%type,
    p_name iam.roles.name%type
)
language plpgsql
as $$
begin
    insert into iam.roles (
        id,
        tenant_id,
        name
    ) values (
        p_id,
        p_tenant_id,
        p_name
    );
end
$$;