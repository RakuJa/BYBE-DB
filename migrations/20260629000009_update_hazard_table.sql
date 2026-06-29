ALTER TABLE pf_hazard_table ALTER COLUMN ac DROP NOT NULL;

ALTER TABLE pf_hazard_table ALTER COLUMN hp DROP NOT NULL;

ALTER TABLE pf_hazard_table ADD COLUMN hp_details TEXT;

--

ALTER TABLE sf_hazard_table ALTER COLUMN ac DROP NOT NULL;

ALTER TABLE sf_hazard_table ALTER COLUMN hp DROP NOT NULL;

ALTER TABLE sf_hazard_table ADD COLUMN hp_details TEXT;