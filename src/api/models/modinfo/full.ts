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
