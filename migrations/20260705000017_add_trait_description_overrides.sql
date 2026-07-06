-- Manually curated descriptions for traits missing them in the source dataset.
-- Used to be copied if parsed descriptions are empty
CREATE TABLE IF NOT EXISTS pf_trait_description_overrides (
    trait_name  TEXT PRIMARY KEY NOT NULL,
    description TEXT NOT NULL
);

INSERT INTO pf_trait_description_overrides (trait_name, description) VALUES
('whetstone', 'Whetstones are magically enhanced items designed to increase a weapon’s effectiveness, sometimes taking the form of literal stones, but often instead constructed of materials with a metaphorical link to the whetstone’s effect. They’re similar to talismans, though unlike talismans, they don’t need to be Affixed ahead of time.'),
('shade', 'Shades are mortal souls who have been judged and then transformed into creatures native to other planes. Petitioners can survive the basic environmental effects of their home plane.'),
('locathah', 'Athamarus are fishlike aquatic humanoids.'),
('naari', 'Naaris are planar scions descended from efreet.'),
('lilu', 'All lilus are chaotic evil fiends who can fly, are immune to disease, have mixed avian and humanoid appearances, possess darkvision and lifesense, and resist physical damage (except from cold iron). Most have a selection of primal innate spells (often including a primal version of dimension door)'),
('harrow-court', 'This trait is associated with the Deck of Destiny and the Harrow Court. It is limited to the Stolen Fate campaign.'),
('couatl', 'A family of supernatural feathered serpents who serve as guardians and messengers in the Universe for various holy divinities.'),
('petitioner', 'Petitioners are mortal souls who have been judged and then transformed into creatures native to other planes. Petitioners can survive the basic environmental effects of their home plane.'),
('rakshasa', 'Rakshasas are divine spirits who exemplify the profane. They can typically Change Shape and have a weakness to holy.'),
('jotunborn', 'A creature with this trait is a member of the jotunborn ancestry. Jotunborn are large people forged from the blood of an ancient group of giants called titans and usually have low-light vision. An ability with this trait can be used or selected only by jotunborn. A weapon with this trait is created and used by jotunborn.'),
('wild-hunt', 'Creatures with the wild hunt trait are never below 14th level. They possess greater darkvision, greensight, planar acclimation, wild gaze, instinctive cooperation, weakness to cold iron, the ability to speak with plants and animals, and their own specific wild hunt link.'),
('hungerseed', 'A name for those who have oni ancestry.'),
('drow', 'Subterranean kin of the elves, drow typically have darkvision and inborn magical abilities.'),
('golem', 'Golems are a special type of construct. Golems are immune to almost all magic, but most have a weakness to certain spells.'),
('gigas', 'One of the first creations of the titans. The few existing stories of their origin suggest they were created from the blood of a titan who was injured in a mighty battle. Amnerion then carved their own domain from the caves beneath Iblydos and guards it jealously.'),
('duergar', 'Subterranean kin of the dwarves, hryngars typically have darkvision, resist poison, and recoil from bright light.'),
('barding', '<p>You can purchase special armor for animals, called barding (shown on the Barding table). All animals have a trained proficiency rank in light barding, and combat-trained animals are trained in heavy barding. Barding uses the same rules as armor except for the following. The Price and Bulk of barding depend on the animal’s size (use the statistics for Small barding for creatures below Small size). Barding can’t be etched with magic runes, though special magical barding might be available.</p><table class="pf2e"><h2>Barding</h2><thead><tr><th>Size</th><th>Barding</th><th>Price</th><th>AC Bonus</th><th>Dex Cap</th><th>Check Penalty</th><th>Speed Penalty</th><th>Bulk</th><th>Strength</th></tr></thead><tbody><tr><td rowspan="2">Light</td><td>Small or Medium</td><td>10 gp</td><td>+1</td><td>+5</td><td>-1</td><td>-5 ft.</td><td>2</td><td>+3</td></tr><tr><td>Large</td><td>20 gp</td><td>+1</td><td>+5</td><td>-1</td><td>-5 ft.</td><td>4</td><td>+3</td></tr><tr><td rowspan="2">Heavy</td><td>Small or Medium (Level 2)</td><td>25 gp</td><td>+3</td><td>+3</td><td>-3</td><td>-10 ft.</td><td>4</td><td>+5</td></tr><tr><td>Large (Level 3)</td><td>50 gp</td><td>+3</td><td>+3</td><td>-3</td><td>-10 ft.</td><td>8</td><td>+5</td></tr></tbody></table>')
ON CONFLICT (trait_name) DO UPDATE SET description = EXCLUDED.description;


CREATE TABLE IF NOT EXISTS sf_trait_description_overrides (
    trait_name  TEXT PRIMARY KEY NOT NULL,
    description TEXT NOT NULL
);

INSERT INTO sf_trait_description_overrides (trait_name, description) VALUES
    ('shobhad', 'A creature with this trait is a member of the shobhad ancestry. An ability with this trait can be used or selected only be shobhads.'),
    ('midwife', 'Aberrations with a connection to the Newborn.'),
    ('vesk', 'A creature with this trait is a member of the vesk ancestry. Vesk are proud reptilian conquerers with a strong desire to prove their strength. An ability with this trait can be used or selected only by vesk.'),
    ('elindrian', 'These creatures from an industrialized moon resemble formians corrupted by pollution, share a telepathic bond that allows them to communicate nearly instantaneously, and share a primal connection to a massive radioactive ooze known as the Terror of Ukulam.'),
    ('traversal', 'These abilities can easily be adapted for other forms of movement. An action with this trait includes one or more Stride actions but can be easily adapted to other modes of movement. You can substitute a Burrow, Climb, Fly, or Swim action in place of any Stride actions if you have the corresponding movement type. (Any adjustments to the Stride, such as gaining a bonus or penalty to your Speed, still apply to the substituted movement type.)'),
    ('solarian', 'This indicates abilities from the solarian class.'),
    ('drift', 'Creatures with this trait have a connection to the drift and typically have the cosmic trait.'),
    ('jinsul', 'Hierarchical aliens from the Scoured Stars system.'),
    ('osharu', 'A creature with this trait is a member of the osharu ancestry. They are sluglike humanoids that can swim and excrete sticky slime. They spend their lives amassing knowledge. An ability with this trait can be used or selected only by osharus.'),
    ('dragonkin', 'A creature with this trait is a member of the dragonkin ancestry. An ability with this trait can be used or selected only by dragonkin.'),
    ('ysoki', 'A creature with this trait is a member of the ysoki ancestry. Ysoki are also known as ratfolk and are known for their ingenuity and comradery. An ability with this trait can be used or selected only by ratfolk or ysoki.'),
    ('contemplative', 'A creature with this trait is a member of the contemplative ancestry. An ability with this trait can be used or selected only by contemplatives.'),
    ('lashunta', 'A creature with this trait is a member of the lashunta ancestry. Lashunta are charming telepaths known for their unique dimorphism. An ability with this trait can be used or selected only by lashuntas.'),
    ('orocoran', 'A creature with this trait is a member of the orocoran ancestry. They are spindly aberrations mutated by the eldritch fluids of Aucturn who yearn for a new identity. An ability with this trait can be used or selected only by orocorans.'),
    ('shirren', 'A creature with this trait is a member of the shirren ancestry. Shirren are telepathic insect-like people searching for meaning after freeing themselves from the Swarm hive mind. An ability with this trait can be used or selected only by shirrens.'),
    ('mystic', 'This indicates abilities from the mystic class.'),
    ('modular-c-or-void', 'The weapon has multiple configurations that you can switch between using an Interact action. Typically, switching between configurations of a modular weapon allows it to deal different types of damage (listed in the trait, such as “modular B, P, or S”), though it''s possible for a modular weapon''s description to list most complicated configurations'),
    ('sarcesian', 'A creature with this trait is a member of the sarcesian ancestry. An ability with this trait can be used or selected only by Sarcesian.'),
    ('vlaka', 'A creature with this trait is a member of the vlaka ancestry. An ability with this trait can be used or selected only by Vlaka.'),
    ('spectra', 'Servitors of Triune usually found in the Drift.'),
    ('kasatha', 'A creature with this trait is a member of the kasatha ancestry. Kasatha are four-armed migrants from a dying world who value cosmic balance and tradition. An ability with this trait can be used or selected only by kasathas.'),
    ('modular-s-or-void', 'The weapon has multiple configurations that you can switch between using an Interact action. Typically, switching between configurations of a modular weapon allows it to deal different types of damage (listed in the trait, such as “modular B, P, or S”), though it''s possible for a modular weapon''s description to list most complicated configurations'),
    ('kucharn', 'A ravenous insectile species from the Vast also known as the Swarm.'),
    ('witchwarper', 'This indicates abilities from the witchwarper class.')

ON CONFLICT (trait_name) DO UPDATE SET description = EXCLUDED.description;