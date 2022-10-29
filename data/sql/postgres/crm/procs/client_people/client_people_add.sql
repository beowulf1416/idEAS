create or replace procedure client_people_add(
    p_client_id crm.client_people.client_id%type,
    p_people_id crm.client_people.people_id%type
)
language plpgsql
as $$
begin
    insert into crm.client_people (
        client_id,
        people_id
    ) values (
        p_client_id,
        p_people_id
    );
end
$$;

comment on procedure client_people_add (crm.client_people.client_id%type, crm.client_people.people_id%type) is 'associates people records to client';