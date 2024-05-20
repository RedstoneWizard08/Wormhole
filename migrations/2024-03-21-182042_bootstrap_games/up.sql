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
