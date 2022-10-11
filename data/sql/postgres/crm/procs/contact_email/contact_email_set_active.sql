create or replace procedure contact_email_set_active(
    p_id crm.contact_email.id%type,
    p_active crm.contact_email.active%type
)
language plpgsql
as $$
begin
    update crm.contact_email set
        active = p_active
    where
        id = p_id
    ;
end
$$;