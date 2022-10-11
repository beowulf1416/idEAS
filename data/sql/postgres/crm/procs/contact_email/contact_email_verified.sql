create or replace procedure contact_email_verified(
    p_id crm.contact_emails.id%type
)
language plpgsql
as $$
begin
    update crm.contact_emails set
        verified = now() at time zone 'utc'
    where
        id = p_id
    ;
end
$$;