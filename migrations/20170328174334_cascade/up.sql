ALTER TABLE answer DROP COLUMN question_id;
ALTER TABLE answer ADD COLUMN question_id INT NOT NULL REFERENCES question ON DELETE CASCADE ;

ALTER TABLE question DROP COLUMN category_id;
ALTER TABLE question ADD COLUMN category_id INT NOT NULL REFERENCES category ON DELETE CASCADE ;