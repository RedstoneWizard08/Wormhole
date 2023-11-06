import { SpaceDockAPI } from "../SpaceDock";

export const getMod = async (modId?: string) => {
    const api = new SpaceDockAPI();

    return await api.getMod(modId || "");
};

export const getMods = async (page?: number, count?: number, gameId?: number) => {
    const api = new SpaceDockAPI();

    return gameId ? await api.getModsForGame(gameId, page, count) : await api.getMods(page, count);
};
