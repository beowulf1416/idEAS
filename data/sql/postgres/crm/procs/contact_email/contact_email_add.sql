create or replace procedure contact_email_add(
    p_id crm.contact_email.id%type,
    p_people_id crm.contact_email.people_id%type,
    p_email crm.contact_email.email%type
)
language plpgsql
as $$
begin
    insert into crm.contact_email values (
        id,
        people_id,
        email
    ) values (
        p_id,
        p_people_id,
        p_email
    );
end
$$;