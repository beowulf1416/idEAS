create or replace function permission_get_id(
    p_name iam.permissions.name%type
)
returns iam.permissions.id%type
language plpgsql
as $$
declare
    t_permission_id iam.permissions.id%type;
begin
    select
        a.id into t_permission_id
    from iam.permissions a
    where
        a.name = p_name
    ;

    return t_permission_id;
end
$$;