ALTER TABLE answer DROP CONSTRAINT answer_text_question_id_key;
ALTER TABLE answer ADD CONSTRAINT  answer_text_key UNIQUE(text);