create table contact_emails(
    id uuid not null,
    active boolean not null default true,
    created timestamp without time zone not null default(now() at time zone 'utc'),
    people_id uuid not null,

    email common.email_address not null,
    verified boolean not null default false,

    constraint pk_contact_email
        primary key (id),

    constraint u_contact_email_1
        unique (email),
    
    constraint fk_contact_email_1
        foreign key (people_id)
        references crm.contact_email (people_id)
        on delete restrict
);