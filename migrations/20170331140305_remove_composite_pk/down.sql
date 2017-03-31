DROP TABLE round_question;

CREATE TABLE round_question (
    id SERIAL NOT NULL,
    round_id INT NOT NULL REFERENCES round ON UPDATE CASCADE ON DELETE CASCADE,
    question_id INT NOT NULL REFERENCES question ON UPDATE CASCADE,
    start_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    end_time TIMESTAMP NULL DEFAULT CURRENT_TIMESTAMP,
    is_joker_used BOOLEAN NOT NULL DEFAULT FALSE,
    CONSTRAINT round_question_pkey PRIMARY KEY (round_id, question_id)
);