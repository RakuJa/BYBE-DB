ALTER TABLE pf_creature_table  ADD COLUMN status TEXT NOT NULL DEFAULT 'valid';
ALTER TABLE pf_item_table  ADD COLUMN status TEXT NOT NULL DEFAULT 'valid';

ALTER TABLE sf_creature_table  ADD COLUMN status TEXT NOT NULL DEFAULT 'valid';
ALTER TABLE sf_item_table  ADD COLUMN status TEXT NOT NULL DEFAULT 'valid';