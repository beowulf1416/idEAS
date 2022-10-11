create or replace procedure contact_email_set_active(
    p_id crm.contact_emails.id%type,
    p_active crm.contact_emails.active%type
)
language plpgsql
as $$
begin
    update crm.contact_emails set
        active = p_active
    where
        id = p_id
    ;
end
$$;