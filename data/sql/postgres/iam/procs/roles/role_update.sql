create or replace procedure role_update(
    p_id iam.roles.id%type,
    p_domain_id iam.roles.domain_id%type,
    p_name iam.roles.name%type,
    p_slug iam.roles.slug%type
)
language plpgsql
as $$
begin
    update iam.roles set
        name = p_name,
        slug = p_slug
    where
        id = p_id
        and domain_id = p_domain_id
    ;
end
$$;