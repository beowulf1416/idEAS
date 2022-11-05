create or replace function role_get_id(
    p_client_id iam.roles.client_id%type,
    p_name iam.roles.name%type
)
returns iam.roles.id%type
language plpgsql
as $$
declare
    t_role_id iam.roles.id%type;
begin
    select
        a.id into t_role_id
    from iam.roles a
    where
        a.name = p_name
    ;

    return t_role_id;
end
$$;