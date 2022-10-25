create or replace procedure role_add(
    p_id iam.roles.id%type,
    p_client_id iam.roles.client_id%type,
    p_name iam.roles.name%type,
    p_description iam.roles.description%type
)
language plpgsql
as $$
begin
    insert into iam.roles (
        id,
        client_id,
        name,
        description
    ) values (
        p_id,
        p_client_id,
        p_name,
        p_description
    );
end
$$;