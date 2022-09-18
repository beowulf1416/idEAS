create table account_balances (
    account_id uuid not null,

    balance decimal(12, 3) not null default 0,

    constraint pk_account_balances
        primary key (account_id),

    constraint fk_account_balances_1
        foreign key (account_id)
        references accounting.accounts (id)
        on delete restrict
);