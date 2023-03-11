import "./Browse.scss";
import { useEffect, useState } from "preact/compat";
import { Mod } from "../../components/Mod";
import { Pagination } from "../../components/Pagination";
import {
    BrowseModInfo,
    ModWithDistance,
} from "../../api/models/modinfo/browse";
import { invoke_proxy } from "../../invoke";
import { SearchBar } from "../../components/SearchBar";

export const Browse = () => {
    const [results, setResults] = useState<BrowseModInfo[]>([]);
    const [perPage] = useState(30);
    const [pages, setPages] = useState(1);
    const [page, setPage] = useState(1);
    const [loading, setLoading] = useState(true);
    const [initialLoad, setInitialLoad] = useState(true);

    useEffect(() => {
        (async () => {
            setLoading(true);

            const data = await invoke_proxy("get_mods", {
                gameId: 22407,
                count: perPage,
                page,
            });

            setResults(data.result);
            setPages(data.pages);

            if (initialLoad) setInitialLoad(false);

            setLoading(false);
        })();
    }, [page, initialLoad, perPage]);

    async function searchMods(
        mods: BrowseModInfo[],
        query: string
    ): Promise<BrowseModInfo[]> {
        console.log();
        const test_data = await invoke_proxy("get_distance", {
            query,
            modName: mods[0].name,
        });
        console.log(test_data);
        const exactMatches: BrowseModInfo[] = [];
        const closeMatches: ModWithDistance[] = [];

        const regex = new RegExp(`^${query}`, "i");

        for (const mod of mods) {
            if (
                mod.name.toLowerCase() === query.toLowerCase() ||
                regex.test(mod.name)
            ) {
                exactMatches.push(mod);
            } else {
                const dist = await invoke_proxy("get_distance", {
                    query,
                    modName: mod.name,
                });
                closeMatches.push({ mod, dist });
            }
        }

        closeMatches.sort((a, b) => a.dist! - b.dist!);

        return exactMatches.concat(closeMatches.map((m) => m.mod));
    }

    const handleSearch = async (query: string) => {
        console.log("Searching for:", query);

        const matches = await searchMods(results, query);

        console.log(matches);

        setResults(matches);
    };

    return (
        <>
            <div className="browse-container">
                {!initialLoad ? (
                    <Pagination pages={pages} page={page} setPage={setPage} />
                ) : (
                    <></>
                )}

                {loading ? (
                    <p className="loading">Loading...</p>
                ) : (
                    <>
                        <div className="pagination-search-bar">
                            <SearchBar onSearch={handleSearch} />
                        </div>
                        <div className="grid">
                            {results.map((mod) => (
                                <Mod mod={mod} key={mod.id} />
                            ))}
                        </div>
                    </>
                )}
            </div>
        </>
    );
};
