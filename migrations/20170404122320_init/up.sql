create table answer
(
  id serial not null
    constraint answer_pkey
    primary key,
  text text not null,
  is_correct boolean,
  question_id integer not null,
  is_active boolean default true not null,
  constraint answer_text_question_id_key
  unique (text, question_id)
)
;

create table category
(
  id serial not null
    constraint category_pkey
    primary key,
  text text not null
    constraint category_text_key
    unique,
  is_active boolean default true not null
)
;

create table question
(
  id serial not null
    constraint question_pkey
    primary key,
  text text not null
    constraint question_text_key
    unique,
  category_id integer not null
    constraint question_category_id_fkey
    references category
    on delete cascade,
  is_active boolean default true not null
)
;

alter table answer
  add constraint answer_question_id_fkey
foreign key (question_id) references question
on delete cascade
;

create table round
(
  id serial not null
    constraint round_pkey
    primary key,
  category_id integer not null
    constraint round_category_id_fkey
    references category,
  player_id integer not null
)
;

create table player
(
  id serial not null
    constraint player_pkey
    primary key,
  name varchar(15) not null
)
;

alter table round
  add constraint round_player_id_fkey
foreign key (player_id) references player
on delete cascade
;

create table round_question
(
  id serial not null
    constraint round_question_pkey
    primary key,
  round_id integer not null
    constraint round_question_round_id_fkey
    references round
    on update cascade on delete cascade,
  question_id integer not null
    constraint round_question_question_id_fkey
    references question
    on update cascade,
  start_time timestamp default now() not null,
  end_time timestamp default now(),
  is_joker_used boolean default false not null,
  constraint round_question_round_id_question_id_key
  unique (round_id, question_id)
)
;

create table user_account
(
  id serial not null
    constraint user_account_pkey
    primary key,
  email varchar(254) not null
    constraint user_account_email_key
    unique,
  name varchar(254) not null
    constraint user_account_name_key
    unique,
  password text not null
)
;
