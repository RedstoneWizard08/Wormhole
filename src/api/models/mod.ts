import { ModSource } from "./src";

export interface InstalledMod {
    name: string;
    id: number;
    size: number;
    source: ModSource;
    file: string;
    version: number;
}

export const demoMods: InstalledMod[] = [
    {
        name: "Space Warp + BepInEx",
        size: 37220000,
        id: 3277,
        source: ModSource.SpaceDock,
        file: "Space_Warp__BepInEx-1.9.4.zip",
        version: 20780,
    },
];
