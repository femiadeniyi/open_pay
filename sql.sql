create table if not exists pat
(
    id varchar(100) not null,
    constraint pat_id_uindex
        unique (id)
);

alter table pat
    add primary key (id);

create table if not exists person
(
    id             int auto_increment
        primary key,
    first_name     varchar(50) not null,
    last_name      varchar(50) not null,
    buyer          tinyint(1)  not null,
    seller         tinyint(1)  not null,
    sort_code      varchar(10) not null,
    account_number varchar(10) not null
);

create table if not exists profile
(
    id varchar(50) not null
        primary key
);

create table if not exists bank
(
    id         int auto_increment
        primary key,
    pat_id     varchar(100) not null,
    profile_id varchar(50)  not null,
    constraint bank_pat_id_fk
        foreign key (pat_id) references pat (id),
    constraint bank_profile_id_fk
        foreign key (profile_id) references profile (id)
);

create table if not exists reference
(
    id   int auto_increment
        primary key,
    name varchar(50) not null
);

create table if not exists transaction_status
(
    id     int auto_increment
        primary key,
    status varchar(50) not null
);

create table if not exists transaction
(
    id                    int auto_increment
        primary key,
    reference_id          int                                not null,
    buyer_id              int                                not null,
    seller_id             int                                not null,
    transaction_status_id int                                not null,
    timestamp             datetime default CURRENT_TIMESTAMP not null,
    constraint transaction_person_id_fk
        foreign key (buyer_id) references person (id),
    constraint transaction_person_id_fk_2
        foreign key (seller_id) references person (id),
    constraint transaction_reference_id_fk
        foreign key (reference_id) references reference (id),
    constraint transaction_transaction_status_id_fk
        foreign key (transaction_status_id) references transaction_status (id)
);

