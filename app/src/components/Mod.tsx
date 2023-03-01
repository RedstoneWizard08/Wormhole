import "./Mod.scss";
import { FunctionalComponent } from "preact";

export interface ModVersion {
    friendly_version: string;
    game_version: string;
    id: number;
    created: string;
    download_path: string;
    changelog: string;
    downloads: number;
}

export interface ModInfo {
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

export interface ModParams {
    info: Partial<ModInfo>;
}

export const fixVersions = (raw: Partial<ModVersion>[]): ModVersion[] => {
    return raw.map((version) => ({
        friendly_version: version.friendly_version || "",
        game_version: version.friendly_version || "",
        id: version.id || 0,
        created: version.created || "",
        download_path: version.download_path || "",
        changelog: version.changelog || "",
        downloads: version.downloads || 0,
    }));
};

export const finishModInfo = (raw: Partial<ModInfo>): ModInfo => {
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
        background: raw.background || "https://spacedock.info/static/background-s.png",
        bg_offset_y: raw.bg_offset_y || "",
        license: raw.license || "",
        website: raw.website || "",
        donations: raw.donations || "",
        source_code: raw.source_code || "",
        url: raw.url || "",
        versions: raw.versions ? fixVersions(raw.versions) : [],
    };
};

export const Mod: FunctionalComponent<ModParams> = ({ info }) => {
    const mod = finishModInfo(info);

    const capText = (text: string, size: number) => {
        if (text.length > size)
            return text.substring(0, size - 3) + "...";
        
        return text;
    };

    return (
        <div className="mod">
            <img src={mod.background} class="image" />

            <p class="title">{capText(mod.name, 26)}</p>
        </div>
    );
};
