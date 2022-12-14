create or replace function item_balance_fetch(
    p_client_id inventory.item_balances.client_id%type,
    p_item_id inventory.item_balances.item_id%type,
)
returns table (
    active inventory.item_balances.active%type,
    balance inventory.item_balances.balance%type
)
language plpgsql
as $$
begin
    return query
    select
        a.active,
        a.balance
    from inventory.item_balances a
    where
        a.client_id = p_client_id
        and a.item_id = p_item_id
    ;
end
$$;