create or replace procedure contact_email_verified(
    p_id crm.contact_email.id%type
)
language plpsql
as $$
begin
    update crm.contact_email set
        verified = now() at time zone 'utc'
    where
        id = p_id
    ;
end
$$;