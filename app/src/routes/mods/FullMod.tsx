import "./FullMod.scss";
import { Link, useRouter } from "preact-router";
import { useEffect, useState } from "preact/hooks";
import { marked } from "marked";
import { FullModInfo } from "../../api/models/modinfo/full";
import { invoke_proxy } from "../../invoke";
import DOMPurify from "dompurify";
import { LoadingPage } from "../../components/LoadingPage";
import { KSPGame } from "../../api/instance";

export const FullMod = () => {
    const [router] = useRouter();
    const modId = router.matches?.mod;

    const [modInfo, setModInfo] = useState<FullModInfo | undefined>(undefined);
    const [isLoading, setIsLoading] = useState(true);
    const [mods, setMods] = useState(false);

    useEffect(() => {
        setMods(/\/mods?(\/\d+)?/i.test(router.path!));
        (async () => {
            const mod = await invoke_proxy("get_mod", {
                modId: parseInt(modId || "-1", 10),
                gameId: KSPGame.KSP2,
            });
            
            setModInfo(mod);
            setIsLoading(false);
        })();
    }, [modId, router.path]);

    const linkFix = (html: string) => {
        const linkRegex =
            /(<a\s+(?!.*\btarget=)[^>]*)(href="https?:\/\/)(.*?")/gi;
        const matches = html.matchAll(linkRegex);

        for (const match of matches) {
            const startTag = match[1];
            const href = match[2] + match[3];
            const newStartTag = `${startTag} target="_blank"`;
            const newLink = `${newStartTag} ${href}`;
            html = html.replace(match[0], newLink);
        }

        return html;
    };

    const imageFix = (html: string) => {
        const parser = new DOMParser();
        const doc = parser.parseFromString(html, "text/html");
        const images = doc.querySelectorAll("img");

        images.forEach((img) => {
            img.style.maxWidth = "50%";
        });

        return doc.body.innerHTML;
    };

    const handleHtml = (html: string) => {
        const processes = [marked, DOMPurify.sanitize, linkFix, imageFix];

        html = processes.reduce((html, process) => process(html), html);
        return html;
    };

    const install = async () => {
        const downloadUrl = await invoke_proxy("get_mod_download", {
            modId: parseInt(modId || "-1", 10),
            gameId: KSPGame.KSP2,
        });

        void downloadUrl;
    };

    if (isLoading) {
        return <LoadingPage />;
    }

    return (
        <div className="full-mod-container">
            <Link className={`link ${mods ? "active" : ""}`} href={"/mods"}>
                <div className="return-container">
                    <div className="return-arrow">
                        <i className="fa-solid fa-long-arrow-left" />
                    </div>
                    <div className="return-circle" />
                </div>
            </Link>
            <div className="mod">
                <img
                    src={modInfo?.background}
                    className="background"
                    alt="mod-background-image"
                />

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
                        __html: handleHtml(
                            modInfo?.description || "Loading description..."
                        ),
                    }}
                />
            </div>

            <div className="actions">
                <button type="button" className="action" onClick={install}>
                    <i className="icon fa-regular fa-circle-down" />
                    &nbsp; Install
                </button>
            </div>
        </div>
    );
};
