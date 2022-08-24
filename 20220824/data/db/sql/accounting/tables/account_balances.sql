create table account_balances (
    account_id uuid not null,
    amount numeric(12, 4) not null default 0,

    constraint pk_account_balances
        primary key (account_id),
    constraint fk_account_balances_1
        foreign key (account_id)
        references accounting.accounts (id)
        on delete restrict
);