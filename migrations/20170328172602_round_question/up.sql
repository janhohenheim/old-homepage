CREATE TABLE round_question (
  round_id INT NOT NULL REFERENCES round ON UPDATE CASCADE ON DELETE CASCADE,
  question_id INT NOT NULL REFERENCES question ON UPDATE CASCADE,
  CONSTRAINT round_question_pkey PRIMARY KEY (round_id, question_id),

  start_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  end_time TIMESTAMP NULL
);

ALTER TABLE round DROP COLUMN start_time;
ALTER TABLE round DROP COLUMN end_time;
