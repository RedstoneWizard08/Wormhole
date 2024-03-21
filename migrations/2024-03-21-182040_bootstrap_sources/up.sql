-- ID is constant here to match up with the enum.
INSERT INTO sources (id, name, base_url) VALUES
    (0, "SpaceDock", "https://spacedock.info/api"),
    (1, "CKAN", "<ckandex>"),
    (2, "Wormhole", "<TBD>"),
    (3, "Local", "<none>"),
    (4, "CurseForge", "https://api.curseforge.com"),
    (5, "Modrinth", "https://api.modrinth.com"),
    (6, "Thunderstore", "https://thunderstore.io/api"),
    (7, "Nexus Mods", "https://api.nexusmods.com"),
    (8, "Unknown", "<unknown>");
