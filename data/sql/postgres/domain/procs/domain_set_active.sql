create or replace procedure domain_set_active (
    p_id domain.domains.id%type,
    p_active domain.domains.active%type
)
language plpgsql
as $$
begin
    update domain.domains set
        active = p_active
    where
        id = p_id
    ;
end;
$$;