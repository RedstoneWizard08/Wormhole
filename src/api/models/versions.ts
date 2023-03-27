export interface ModVersion {
    friendly_version: string;
    game_version: string;
    id: number;
    created: string;
    download_path: string;
    changelog: string;
    downloads: number;
}

export const finishVersions = (raw: Partial<ModVersion>[]): ModVersion[] => {
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
