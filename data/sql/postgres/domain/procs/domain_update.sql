create or replace procedure domain_update (
    p_id domain.domains.id%type,
    p_name domain.domains.name%type,
    p_slug domain.domains.slug%type
)
language plpgsql
as $$
begin
    update domain.domains set
        name = p_name,
        slug = p_slug
    where
        id = p_id
    ;
end
$$;