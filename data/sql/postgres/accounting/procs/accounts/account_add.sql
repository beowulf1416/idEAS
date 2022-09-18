create or replace procedure account_add(
    p_client_id accounting.accounts.client_id%type,
    p_account_id accounting.accounts.id%type,
    p_type_id accounting.accounts.type_id%type,
    p_name accounting.accounts.name%type,
    p_description accounting.accounts.description%type 
)
language plpgsql
as $$
begin
    insert into accounting.accounts (
        id,
        client_id,
        active,
        type_id,
        name,
        description
    ) values (
        p_account_id,
        p_client_id,
        p_type_id,
        p_name,
        p_description
    );

    insert into accounting.account_balances (
        account_id,
        balance
    ) values (
        p_account_id,
        0
    );
end
$$;