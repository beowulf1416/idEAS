create or replace procedure domain_add(
    p_domain_id domain.domains.id%type,
    p_domain_name domain.domains.name%type,
    p_domain_slug domain.domains.slug%type
)
language plpgsql
as $$
begin
    insert into domain.domains (
        id,
        active,
        name,
        slug
    ) values (
        p_domain_id,
        false,
        p_domain_name,
        p_domain_slug
    );
end
$$;