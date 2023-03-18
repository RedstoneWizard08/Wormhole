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
import { LoadingPage } from "../../components/LoadingPage";
import { Dropdown } from "../../components/Dropdown";
import { useRouter } from "preact-router";

export const Browse = () => {
    const [router] = useRouter();
    const params = new URL(`http://localhost/${router.url}`).searchParams;
    const [results, setResults] = useState<BrowseModInfo[]>([]);
    const [perPage] = useState(30);
    const [pages, setPages] = useState(1);
    const [page, setPage] = useState(1);
    const [loading, setLoading] = useState(true);
    const [initialLoad, setInitialLoad] = useState(true);
    const [gameId, setGameId] = useState(
        parseInt(params.get("game") || "3102", 10)
    );

    // These must be managed outside the
    // dropdown component, otherwise the
    // component's data will reset every
    // time the mod list is refreshed.
    const [game, setGame] = useState(gameId == 3102 ? "ksp1" : "ksp2");
    const [gameText, setGameText] = useState(
        gameId == 3102 ? "KSP 1" : "KSP 2"
    );

    useEffect(() => {
        setGameId(game == "ksp1" ? 3102 : 22407);
    }, [game]);

    useEffect(() => {
        setLoading(true);
        setPages(0);

        (async () => {
            const data = await invoke_proxy("get_mods", {
                gameId,
                count: perPage,
                page,
            });

            setResults(data.result);
            setPages(data.pages);

            if (page > data.pages) setPage(data.pages - 1);
            if (initialLoad) setInitialLoad(false);

            setLoading(false);
        })();
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [page, perPage, gameId]);

    async function searchMods(
        mods: BrowseModInfo[],
        query: string
    ): Promise<BrowseModInfo[]> {
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
                {!initialLoad && pages > 0 ? (
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
                    <LoadingPage />
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
