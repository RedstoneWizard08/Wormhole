import "./Browse.scss";
import {useEffect, useState} from "preact/compat";
import {Mod} from "../../components/Mod";
import {Pagination} from "../../components/Pagination";
import {BrowseModInfo} from "../../api/models/modinfo/browse";
import {invoke_proxy} from "../../invoke";
import {SearchBar} from "../../components/SearchBar";

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

    function levenshteinDistance(a: string | any[], b: string | any[]) {
        const m = a.length;
        const n = b.length;
        const matrix = [];

        if (m === 0) return n;
        if (n === 0) return m;

        for (let i = 0; i <= m; i++) {
            matrix[i] = [i];
        }
        for (let j = 0; j <= n; j++) {
            matrix[0][j] = j;
        }

        for (let i = 1; i <= m; i++) {
            for (let j = 1; j <= n; j++) {
                const cost = a[i - 1] === b[j - 1] ? 0 : 1;
                matrix[i][j] = Math.min(
                    matrix[i - 1][j] + 1,
                    matrix[i][j - 1] + 1,
                    matrix[i - 1][j - 1] + cost
                );
            }
        }
        return matrix[m][n];
    }


    function searchMods(mods: BrowseModInfo[], query: string): BrowseModInfo[] {
        setLoading(true);
        const exactMatches: BrowseModInfo[] = [];
        const closeMatches: BrowseModInfo[] = [];

        const regex = new RegExp(`^${query}`, 'i');

        mods.forEach((mod) => {
            if (mod.name.toLowerCase() === query.toLowerCase() || regex.test(mod.name)) {
                exactMatches.push(mod);
            } else {
                closeMatches.push(mod);
            }
        });

        closeMatches.sort((a, b) => {
            const distanceA = levenshteinDistance(a.name.toLowerCase(), query.toLowerCase());
            const distanceB = levenshteinDistance(b.name.toLowerCase(), query.toLowerCase());
            return distanceA - distanceB;
        });

        setLoading(false);
        return exactMatches.concat(closeMatches);
    }

    const handleSearch = (query: string) => {
        console.log('Searching for:', query);

        const matches = searchMods(
            results,
            query
        );

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
