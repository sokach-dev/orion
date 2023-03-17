-- Add up migration script here

-- create a story table
CREATE TABLE IF NOT EXISTS orion.story (
    id BIGSERIAL PRIMARY KEY,
    words VARCHAR(64)[] NOT NULL COMMENT 'story key words',
    content TEXT NOT NULL COMMENT 'story content',
    read_count BIGINT NOT NULL DEFAULT 0 COMMENT 'read count',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX orion_story_words_idx ON orion.story USING GIN (words);
CREATE INDEX orion_story_read_count_idx ON orion.story (read_count);

-- create a new word table
CREATE TYPE orion.word_status AS ENUM ('new', 'easy', 'difficult', 'learned');

CREATE TABLE IF NOT EXISTS orion.new_word (
    id BIGSERIAL PRIMARY KEY,
    word VARCHAR(64) NOT NULL COMMENT 'new word',
    vocabulary_id BIGINT COMMENT 'vocabulary id',
    story_id BIGINT[] COMMENT 'associated story',
    learn_count BIGINT NOT NULL DEFAULT 0 COMMENT 'learn count',
    learn_status orion.word_status NOT NULL DEFAULT 'new' COMMENT 'word status',
    last_learned_at TIMESTAMP WITH TIME ZONE COMMENT 'last learned time',
    next_learn_at TIMESTAMP WITH TIME ZONE COMMENT 'next learn time',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX orion_new_word_idx ON orion.new_word (word);
CREATE INDEX orion_new_word_learn_status_idx ON orion.new_word (learn_status);
CREATE INDEX orion_new_word_next_learn_at_idx ON orion.new_word (next_learn_at);
