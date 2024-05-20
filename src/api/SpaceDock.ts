// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

import axios from "axios";

import { finishBrowseResult } from "./models/browse";
import { finishFullModInfo } from "./models/modinfo/full";

export class SpaceDockAPI {
    private base: string;

    public constructor(baseUrl?: string) {
        this.base = baseUrl || SpaceDockAPI.getDefaultAPIUrl();
    }

    public static getDefaultAPIUrl() {
        return import.meta.env.DEV ? "/_spacedock/api" : "https://spacedock.info/api";
    }

    public async getMods(page = 1, count = 30) {
        const response = await axios.get(`${this.base}/browse?page=${page}&count=${count}`);

        return finishBrowseResult(response.data);
    }

    public async getMod(id: string | number) {
        const response = await axios.get(`${this.base}/mod/${id}`);

        return finishFullModInfo(response.data);
    }

    public async getModDownload(id: string | number) {
        const response = await axios.get(`${this.base}/mod/${id}/latest`);

        return (
            (this.base == "/_spacedock/api" ? "/_spacedock" : this.base) +
            response.data.download_path
        );
    }

    public async getModsForGame(gameId: number, page = 1, count = 30) {
        // This is not implemented yet! To look at the implementation, see this PR:
        // KSP-SpaceDock/SpaceDock#466
        const response = await axios.get(
            `${this.base}/browse?page=${page}&count=${count}&game_id=${gameId}`
        );

        return finishBrowseResult(response.data);
    }
}
