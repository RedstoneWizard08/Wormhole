import { BrowseModInfo, finishBrowseModInfo } from "./modinfo/browse";

export interface BrowseResult {
    result: BrowseModInfo[];
    count: number;
    pages: number;
    page: number;
}

export const finishBrowseResult = (data: Partial<BrowseResult>): BrowseResult => {
    return {
        result: data.result ? data.result.map((data) => finishBrowseModInfo(data)) : [],
        count: data.count || 0,
        pages: data.pages || 0,
        page: data.page || 0,
    };
};
