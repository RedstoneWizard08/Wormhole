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
import { Dropdown } from "../../components/Dropdown";

export const Browse = () => {
    const [results, setResults] = useState<BrowseModInfo[]>([]);
    const [perPage] = useState(30);
    const [pages, setPages] = useState(1);
    const [page, setPage] = useState(1);
    const [loading, setLoading] = useState(true);
    const [initialLoad, setInitialLoad] = useState(true);
    const [gameId, setGameId] = useState(3102);

    // These must be managed outside of the
    // dropdown component, otherwise the
    // component's data will reset every
    // time the mod list is refreshed.
    const [game, setGame] = useState("ksp1");
    const [gameText, setGameText] = useState("KSP 1");

    useEffect(() => {
        setGameId(game == "ksp1" ? 3102 : 22407);
    }, [game]);

    useEffect(() => {
        (async () => {
            setLoading(true);

            const data = await invoke_proxy("get_mods", {
                gameId,
                count: perPage,
                page,
            });

            setResults(data.result);
            setPages(data.pages);

            if (initialLoad) setInitialLoad(false);

            setLoading(false);
        })();
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [page, perPage, gameId]);

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

                <div className="top">
                    <Dropdown
                        val={game}
                        setVal={setGame}
                        valText={gameText}
                        setValText={setGameText}
                    />

                    <div className="search-bar">
                        <SearchBar onSearch={handleSearch} />
                    </div>

                    <div className="_blank" />
                </div>

                {loading ? (
                    <p className="loading">Loading...</p>
                ) : (
                    <div className="grid">
                        {results.map((mod) => (
                            <Mod mod={mod} key={mod.id} />
                        ))}
                    </div>
                )}
            </div>
        </>
    );
};
