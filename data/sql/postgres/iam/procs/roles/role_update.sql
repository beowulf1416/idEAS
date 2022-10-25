create or replace procedure role_update(
    p_id iam.roles.id%type,
    p_client_id iam.roles.client_id%type,
    p_name iam.roles.name%type,
    p_description iam.roles.description%type
)
language plpgsql
as $$
begin
    update iam.roles set
        name = p_name,
        description = p_desc
    where
        id = p_id
        and client_id = p_client_id
    ;
end
$$;