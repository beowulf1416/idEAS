create or replace procedure item_balance_subtract(
    p_client_id inventory.item_balances.client_id%type,
    p_item_id inventory.item_balances.item_id%type,
    p_quantity inventory.item_balances.balance%type
)
language plpgsql
as $$
begin
    insert into inventory.item_balances (
        item_id,
        client_id,
        balance
    ) values (
        p_item_id,
        p_client_id,
        p_quantity * -1
    )
    on conflict (item_id)
    do update set
        balance = balance - p_quantity
    ;
end
$$;