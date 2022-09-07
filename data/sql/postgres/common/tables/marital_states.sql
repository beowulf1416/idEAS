create table marital_states (
    id smallint not null,
    name varchar(50) not null,

    constraint pk_marital_status
        primary key (id)
);


insert into marital_states (id, name) values 
(1, 'single'),
(2, 'married'),
(3, 'separated'),
(4, 'divorced'),
(5, 'widowed')
;