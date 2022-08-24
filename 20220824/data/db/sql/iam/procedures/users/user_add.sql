/**
 * add user account
 */
create or replace procedure user_add (
    p_user_id iam.users.id%type,
    p_email iam.users.email%type,
    p_pw iam.users.pw%type
)
language plpgsql
as $$
declare
    t_tenant_id iam.user_tenants.tenant_id%type;
begin
    insert into iam.users (
        id,
        email,
        pw
    ) values (
        p_user_id,
        p_email,
        public.crypt(p_pw, public.gen_salt('md5'))
    );

    -- add user to default tenant
    t_tenant_id := tenants.tenant_default_id();
    call iam.assign_user_to_tenant(
        p_user_id,
        t_tenant_id
    );
end
$$;