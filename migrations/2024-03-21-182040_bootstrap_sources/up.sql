# MIT License
#
# Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
#
# Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

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
