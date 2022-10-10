create or replace function client_default_id()
returns client.clients.id%type
language plpgsql
as $$
declare
    t_client_id client.clients.id%type;
begin
    select
        a.id into t_client_id
    from client.clients a
    where
        name = 'default'
    ;

    return t_client_id;
end
$$;