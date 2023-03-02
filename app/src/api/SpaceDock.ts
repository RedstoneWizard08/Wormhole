import axios from "axios";

import { finishBrowseResult } from "./models/browse";
import { finishFullModInfo } from "./models/modinfo/full";

export class SpaceDockAPI {
    private base: string;

    public constructor(baseUrl?: string) {
        this.base = baseUrl || SpaceDockAPI.getDefaultAPIUrl();
    }

    public static getDefaultAPIUrl() {
        return import.meta.env.DEV ? "/_spacedock" : "https://spacedock.info/api";
    }

    public async getMods(page = 1, count = 30) {
        const response = await axios.get(`${this.base}/browse?page=${page}&count=${count}`);
        
        return finishBrowseResult(response.data);
    }

    public async getMod(id: string | number) {
        const response = await axios.get(`${this.base}/mod/${id}`);

        return finishFullModInfo(response.data);
    }

    public async getModsForGame(gameId: number) {
        const response = await this.getMods();
        const mods = response.result.filter((mod) => mod.game_id == gameId);

        return mods;
    }
}
