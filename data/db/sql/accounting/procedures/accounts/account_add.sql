create or replace procedure account_add (
    p_acct_id accounting.accounts.id%type,
    p_tenant_id accounting.accounts.tenant_id%type,
    p_type_id accounting.accounts.type_id%type,
    p_name accounting.accounts.name%type,
    p_desc accounting.accounts.description%type
)
language plpgsql
as $$
begin
    insert into accounting.accounts (
        id,
        tenant_id,
        type_id,
        name,
        description
    ) values (
        p_acct_id,
        p_tenant_id,
        p_type_id,
        p_name,
        p_desc
    );

    insert into accounting.account_balances (
        account_id,
        amount
    ) values (
        p_acct_id,
        0
    );
end
$$;