// The expected size of KSP1's `steam_api64.dll` in bytes.
// This helps to make sure that the game is not pirated.
// File path: `[KSP1_ROOT]/KSP_x64_Data/Plugins/x86_64/steam_api64.dll`
// Information from: SteamDB, DepotDownloader, KSP1 Installed Files
export const KSP1_STEAM_API_SIZE = 249120;

// The expected size of KSP2's `steam_api64.dll` in bytes.
// This helps to make sure that the game is not pirated.
// File path: `[KSP2_ROOT]/KSP2_x64_Data/Plugins/x86_64/steam_api64.dll`
// Information from: SteamDB, DepotDownloader, KSP2 Installed Files
export const KSP2_STEAM_API_SIZE = 295336;

// This uses the game's ID in SpaceDock as the enum value
export enum KSPGame {
    // eslint-disable-next-line no-unused-vars
    KSP1 = 3102,

    // eslint-disable-next-line no-unused-vars
    KSP2 = 22407,
}

export interface InstanceMod {
    id: number;
    name: string;
    paths: string[];
}

export interface InstanceInfo {
    id: number;
    name: string;
    game: KSPGame;
    mods: InstanceMod[];
    install_path: string;
    description?: string;
    timePlayed?: string;
}
