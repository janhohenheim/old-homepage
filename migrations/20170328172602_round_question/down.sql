DROP TABLE round_question;

ALTER TABLE round ADD COLUMN start_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP;
ALTER TABLE round ADD COLUMN end_time TIMESTAMP NULL;
