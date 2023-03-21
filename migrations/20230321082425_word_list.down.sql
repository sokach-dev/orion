-- Add down migration script here

DROP TABLE orion.word_list CASCADE;
DROP TYPE orion.word_classification;
DROP INDEX orion_word_list_idx;
