ALTER TABLE pf_creature_table ADD COLUMN description TEXT NOT NULL DEFAULT '';
ALTER TABLE sf_creature_table ADD COLUMN description TEXT NOT NULL DEFAULT '';

ALTER TABLE pf_spell_table ADD COLUMN description TEXT NOT NULL DEFAULT '';
ALTER TABLE sf_spell_table ADD COLUMN description TEXT NOT NULL DEFAULT '';
