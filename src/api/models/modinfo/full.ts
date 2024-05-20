// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

import { finishVersions, type ModVersion } from "../versions";

export interface FullModInfo {
    name: string;
    id: number;
    game: string;
    game_id: number;
    short_description: string;
    downloads: number;
    followers: number;
    author: string;
    default_version_id: number;
    shared_authors: unknown[];
    background: string;
    bg_offset_y: string;
    license: string;
    website: string;
    donations: string;
    source_code: string;
    url: string;
    versions: ModVersion[];
    description: string;
}

export const finishFullModInfo = (raw: Partial<FullModInfo>): FullModInfo => {
    return {
        name: raw.name || "",
        id: raw.id || 0,
        game: raw.game || "",
        game_id: raw.game_id || 0,
        short_description: raw.short_description || "",
        downloads: raw.downloads || 0,
        followers: raw.followers || 0,
        author: raw.author || "",
        default_version_id: raw.default_version_id || 0,
        shared_authors: raw.shared_authors || [],
        description: raw.description || "",

        background: raw.background || "https://spacedock.info/static/background.png",

        bg_offset_y: raw.bg_offset_y || "",
        license: raw.license || "",
        website: raw.website || "",
        donations: raw.donations || "",
        source_code: raw.source_code || "",
        url: raw.url || "",
        versions: raw.versions ? finishVersions(raw.versions) : [],
    };
};
