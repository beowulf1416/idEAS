create or replace procedure contact_email_add(
    p_id crm.contact_emails.id%type,
    p_people_id crm.contact_emails.people_id%type,
    p_email crm.contact_emails.email%type
)
language plpgsql
as $$
begin
    insert into crm.contact_emails (
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