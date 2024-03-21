-- ID is constant here to match up with the enum.
INSERT INTO games (id, name) VALUES
    (3102, "Kerbal Space Program"),
    (22407, "Kerbal Space Program 2"),
    (432, "Minecraft");

INSERT INTO supported_sources (is_supported, source_id, game_id) VALUES
    (TRUE, 0, 3102), -- KSP 1 SpaceDock
    (TRUE, 1, 3102), -- KSP 1 CKAN
    (TRUE, 2, 3102), -- KSP 1 Wormhole
    (TRUE, 3, 3102), -- KSP 1 Local
    (TRUE, 4, 3102), -- KSP 1 CurseForge

    (TRUE, 0, 22407), -- KSP 2 SpaceDock
    (TRUE, 1, 22407), -- KSP 2 CKAN
    (TRUE, 2, 22407), -- KSP 2 Wormhole
    (TRUE, 3, 22407), -- KSP 2 Local
    (TRUE, 4, 22407), -- KSP 2 CurseForge

    (TRUE, 2, 432), -- Minecraft Wormhole
    (TRUE, 3, 432), -- Minecraft Local
    (TRUE, 4, 432); -- Minecraft CurseForge
