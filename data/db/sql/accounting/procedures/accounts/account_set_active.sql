create or replace procedure account_set_active (
    p_acct_id accounting.accounts.id%type,
    p_active accounting.accounts.active%type
)
language plpgsql
as $$
begin
    update accounting.accounts set
        active = p_active
    where
        id = p_acct_id
    ;
end
$$;