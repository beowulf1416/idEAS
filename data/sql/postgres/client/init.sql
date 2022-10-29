set schema 'client';

create procedure initialize()
language plpgsql
as $$
declare
    c_client_id client.clients.id%type := public.gen_random_uuid();
    c_country_id client.clients.country_id%type := 608; -- philippines :D
begin

    -- create default client
    call client.client_add(
        c_client_id,
        'default',
        true,
        'default client',
        'default',
        c_country_id,
        ''
    );
end
$$;

call initialize();
drop procedure initialize;