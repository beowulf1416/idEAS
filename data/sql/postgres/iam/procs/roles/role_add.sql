create or replace procedure role_add(
    p_id iam.roles.id%type,
    p_domain_id iam.roles.domain_id%type,
    p_name iam.roles.name%type,
    p_slug iam.roles.slug%type
)
language plpgsql
as $$
begin
    insert into iam.roles (
        id,
        domain_id,
        name,
        slug
    ) values (
        p_id,
        p_domain_id,
        p_name,
        p_slug
    );
end
$$;