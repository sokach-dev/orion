-- Add up migration script here

-- create a story table
CREATE TABLE IF NOT EXISTS orion.story (
    id BIGSERIAL PRIMARY KEY,
    words VARCHAR(64)[] NOT NULL,
    content TEXT NOT NULL,
    read_count BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX orion_story_words_idx ON orion.story USING GIN (words);
CREATE INDEX orion_story_read_count_idx ON orion.story (read_count);

COMMENT ON COLUMN orion.story.words IS 'story key words';
COMMENT ON COLUMN orion.story.content IS 'story content';
COMMENT ON COLUMN orion.story.read_count IS 'read count';

-- create a new word table
CREATE TYPE orion.word_status AS ENUM ('new', 'easy', 'difficult', 'learned');

CREATE TABLE IF NOT EXISTS orion.new_word (
    id BIGSERIAL PRIMARY KEY,
    word VARCHAR(64) NOT NULL,
    vocabulary_id BIGINT,
    story_id BIGINT[],
    learn_count BIGINT NOT NULL DEFAULT 0,
    learn_status orion.word_status NOT NULL DEFAULT 'new',
    last_learned_at TIMESTAMP WITH TIME ZONE,
    next_learn_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX orion_new_word_idx ON orion.new_word (word);
CREATE INDEX orion_new_word_learn_status_idx ON orion.new_word (learn_status);
CREATE INDEX orion_new_word_next_learn_at_idx ON orion.new_word (next_learn_at);

COMMENT ON COLUMN orion.new_word.word IS 'new word';
COMMENT ON COLUMN orion.new_word.vocabulary_id IS 'vocabulary id';
COMMENT ON COLUMN orion.new_word.story_id IS 'associated story';
COMMENT ON COLUMN orion.new_word.learn_count IS 'learn count';
COMMENT ON COLUMN orion.new_word.learn_status IS 'word status';
COMMENT ON COLUMN orion.new_word.last_learned_at IS 'last learned time';
COMMENT ON COLUMN orion.new_word.next_learn_at IS 'next learn time';
