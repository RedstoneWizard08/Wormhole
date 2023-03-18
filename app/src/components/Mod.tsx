import "./Mod.scss";
import { FunctionalComponent } from "preact";
import { route } from "preact-router";
import { BrowseModInfo } from "../api/models/modinfo/browse";
import { invoke_proxy } from "../invoke";
import { useState } from "preact/hooks";
import { KSPGame } from "../api/instance";

export interface ModParams {
    mod: BrowseModInfo;
}

export const Mod: FunctionalComponent<ModParams> = ({ mod }) => {
    // const [router] = useRouter();

    const [installed, setInstalled] = useState(false);
    const [installing, setInstalling] = useState(false);
    // const [instances, setInstances] = useState(false);

    // useEffect(() => {
    //     setInstances(/\/instances?(\/\d+)?/i.test(router.path!));
    // }, [router.path]);

    const capText = (text: string, size: number) => {
        if (text.length > size) return `${text.substring(0, size - 3)}...`;

        return text;
    };

    const onDownload = async (ev: MouseEvent) => {
        ev.preventDefault();
        ev.stopPropagation();

        setInstalling(true);

        await invoke_proxy("install_mod", {
            modId: mod.id,
            gameId: KSPGame.KSP2,
        });

        setInstalling(false);
        setInstalled(!installed);

        // TODO: add writing to mods.json when mod is installed
    };

    const isInstalled = () => {
        return false;
    };

    return (
        <div className="mod-tile" onClick={() => route(`/mod/${mod.id}`)}>
            <img
                src={mod.background}
                className="image"
                alt="mod-background image"
            />

            <div className="info">
                <p className="title">{capText(mod.name, 22)}</p>

                <button type="button" className="action" onClick={onDownload}>
                    {installing ? (
                        <i className="icon fa-solid fa-spinner fa-spin" />
                    ) : isInstalled() ? (
                        <i className="icon fa-solid fa-trash-can" />
                    ) : (
                        <i className="icon fa-solid fa-circle-down" />
                    )}
                </button>
            </div>
        </div>
    );
};
