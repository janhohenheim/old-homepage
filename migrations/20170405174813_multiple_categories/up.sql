ALTER TABLE round DROP COLUMN category_id;
CREATE TABLE round_category (
  id serial primary key,
  round_id integer not null
    references round
    on update cascade on delete cascade,
  category_id integer not null
    references question
    on update cascade,
  constraint round_category_round_id_category_id_key
  unique (round_id, category_id)
);
