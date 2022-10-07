create or replace procedure user_register_complete(
    p_id iam.user_registration.id%type,
    p_email iam.user_registration.email%type,
    p_given_name crm.people.given_name%type,
    p_middle_name crm.people.middle_name%type,
    p_family_name crm.people.family_name%type,
    p_prefix crm.people.prefix%type,
    p_suffix crm.people.suffix%type
)
language plpgsql
as $$
begin
    update iam.user_registration set
        completed = now() at time zone 'utc'
    where
        id = p_id
        and email = p_email
    ;

    insert into crm.people (
        id,
        client_id,
        active,
        given_name,
        middle_name,
        family_name,
        prefix,
        suffix
    ) values (
        p_id,
        p_client_id,
        true,
        p_given_name,
        p_middle_name,
        p_family_name,
        p_prefix,
        p_suffix
    );
end
$$;