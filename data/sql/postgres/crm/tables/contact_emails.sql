create table contact_emails(
    id uuid not null,
    active boolean not null default true,
    created timestamp without time zone not null default(now() at time zone 'utc'),
    verified timestamp without time zone,

    people_id uuid not null,

    email common.email_address not null,

    constraint pk_contact_email
        primary key (id),

    constraint u_contact_email_1
        unique (email),
    
    constraint fk_contact_email_1
        foreign key (people_id)
        references crm.people (id)
        on delete restrict
);