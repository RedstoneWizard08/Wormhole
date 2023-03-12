import "./FullMod.scss";
import {useRouter} from "preact-router";
import {useEffect, useState} from "preact/hooks";
import {marked} from "marked";
import {FullModInfo} from "../../api/models/modinfo/full";
import {invoke_proxy} from "../../invoke";

// @ts-ignore
import DOMPurify from 'dompurify';

export const FullMod = () => {
    const [router] = useRouter();
    const modId = router.matches?.mod;

    const [modInfo, setModInfo] = useState<FullModInfo | undefined>(undefined);

    useEffect(() => {
        (async () => {
            setModInfo(
                await invoke_proxy("get_mod", {
                    modId: parseInt(modId || "-1", 10),
                })
            );
        })();
    }, [modId]);

    const linkFix = (html: string) => {
        const regex = /(<a.*?>)(.*?)(<\/a>)/gi;
        const matches = html.matchAll(regex);

        for (const match of matches) {
            const startTag = match[1];
            const linkContent = match[2];
            const endTag = match[3];
            const urlRegex = /href=["'](.*?)["']/i;
            const targetRegex = /target=["'](.*?)["']/i;

            // Check if the link has a valid URL and no existing target attribute
            if (urlRegex.test(startTag) && !targetRegex.test(startTag)) {
                const newStartTag = `${startTag} target="_blank"`;
                const newLink = `${newStartTag}${linkContent}${endTag}`;
                html = html.replace(match[0], newLink);
            }
        }

        return html;
    };

    const handleHtml = (html: string) => {

        const processes = [
            marked,
            DOMPurify.sanitize,
            linkFix,
            imageFix
        ];

        html = processes.reduce((html, process) => process(html), html);
        return html
    }

    const install = async () => {
        // window.open(await invoke_proxy("get_mod_download", {
        //     modId: parseInt(modId || "-1", 10),
        // }));

        const downloadUrl = await invoke_proxy("get_mod_download", {
            modId: parseInt(modId || "-1", 10),
        });

        void downloadUrl;
    };

    return (
        <div className="full-mod-container">
            <div className="mod">
                <img src={modInfo?.background} className="background"  alt="mod-background-image" />

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
