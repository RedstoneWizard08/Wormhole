import "./Browse.scss";
import axios from "axios";
import { Suspense, useEffect, useState } from "preact/compat";
import { Mod, ModInfo } from "../../components/Mod";
import { Pagination } from "../../components/Pagination";

const apiUrl = import.meta.env.DEV
    ? "/_spacedock"
    : "https://spacedock.info/api";

export interface SpaceDockResult {
    result: Partial<ModInfo>[];
    count: number;
    pages: number;
    page: number;
}

export const Browse = () => {
    const [results, setResults] = useState<any[]>([]);
    const [perPage, setPerPage] = useState(30);
    const [pages, setPages] = useState(1);
    const [page, setPage] = useState(1);
    const [loading, setLoading] = useState(true);
    const [initialLoad, setInitialLoad] = useState(true);

    const refreshMods = async () => {
        const { data: _data } = await axios.get(
            `${apiUrl}/browse?page=${page}&count=${perPage}`
        );
        const data = _data as SpaceDockResult;

        setResults(data.result as ModInfo[]);
        setPages(data.pages);

        if (initialLoad) setInitialLoad(false);
    };

    useEffect(() => {
        (async () => {
            setLoading(true);
            await refreshMods();
            setLoading(false);
        })();
    }, [page]);

    return (
        <div className="browse-container">
            {!initialLoad ? (
                <Pagination pages={pages} page={page} setPage={setPage} />
            ) : (
                <></>
            )}

            {loading ? (
                <p class="loading">Loading...</p>
            ) : (
                <>
                    <div className="grid">
                        {results.map((mod) => (
                            <Mod info={mod} />
                        ))}
                    </div>
                </>
            )}
        </div>
    );
};
