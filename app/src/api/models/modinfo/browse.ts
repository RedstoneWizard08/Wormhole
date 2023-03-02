import { finishVersions, ModVersion } from "../versions";

export interface BrowseModInfo {
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
}

export const finishBrowseModInfo = (
    raw: Partial<BrowseModInfo>
): BrowseModInfo => {
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

        background:
            raw.background || "https://spacedock.info/static/background-s.png",

        bg_offset_y: raw.bg_offset_y || "",
        license: raw.license || "",
        website: raw.website || "",
        donations: raw.donations || "",
        source_code: raw.source_code || "",
        url: raw.url || "",
        versions: raw.versions ? finishVersions(raw.versions) : [],
    };
};
