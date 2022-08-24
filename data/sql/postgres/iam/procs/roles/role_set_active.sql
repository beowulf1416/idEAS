create or replace procedure role_set_active(
    p_id iam.roles.id%type,
    p_active iam.roles.active%type
)
language plpgsql
as $$
begin
    update iam.roles set
        active = p_active
    where
        id = p_id
    ;
end
$$;