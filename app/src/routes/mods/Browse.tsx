import "./Browse.scss";
import { useEffect, useState } from "preact/compat";
import { Mod } from "../../components/Mod";
import { Pagination } from "../../components/Pagination";
import { SpaceDockAPI } from "../../api/SpaceDock";
import { BrowseModInfo } from "../../api/models/modinfo/browse";

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
            
            const spaceDock = new SpaceDockAPI();
            const data = await spaceDock.getModsForGame(22407, page, perPage);

            setResults(data.result);
            setPages(data.pages);

            if (initialLoad) setInitialLoad(false);
            
            setLoading(false);
        })();
    }, [page, initialLoad, perPage]);

    return (
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
                    <div className="grid">
                        {results.map((mod) => (
                            <Mod mod={mod} key={mod.id} />
                        ))}
                    </div>
                </>
            )}
        </div>
    );
};
