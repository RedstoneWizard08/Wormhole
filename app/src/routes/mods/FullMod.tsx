import "./FullMod.scss";
import { useRouter } from "preact-router";
import { useEffect, useState } from "preact/hooks";
import { marked } from "marked";
import { FullModInfo } from "../../api/models/modinfo/full";
import { SpaceDockAPI } from "../../api/SpaceDock";

export const FullMod = () => {
    const [router, _] = useRouter();
    const modId = router.matches?.mod;

    const [modInfo, setModInfo] = useState<FullModInfo | undefined>(undefined);
    const spaceDock = new SpaceDockAPI();

    const refreshModInfo = async () => {
        const data = await spaceDock.getMod(modId || "");

        setModInfo(data);
    };

    useEffect(() => {
        (async () => {
            await refreshModInfo();
        })();
    }, [modId]);

    return (
        <div className="full-mod-container">
            <div className="mod">
                <img src={modInfo?.background} className="background" />

                <div className="infos">
                    <div className="left">
                        <p className="name">{modInfo?.name}</p>
                        &bull;
                        <p className="author">{modInfo?.author}</p>
                    </div>

                    <div className="right">
                        <p className="downloads">
                            <i className="fa-solid fa-circle-down" />
                            &nbsp;&nbsp;
                            {modInfo?.downloads}
                        </p>

                        <p className="followers">
                            <i className="fa-solid fa-eye" />
                            &nbsp;&nbsp;
                            {modInfo?.followers}
                        </p>
                    </div>
                </div>

                <p
                    className="description"
                    dangerouslySetInnerHTML={{
                        __html: marked(modInfo?.description || ""),
                    }}
                />
            </div>

            <div className="actions">
                <button type="button" className="action">
                    <i className="icon fa-regular fa-circle-down" />
                    &nbsp;
                    Install
                </button>
            </div>
        </div>
    );
};