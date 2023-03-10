import "./Mod.scss";
import { FunctionalComponent } from "preact";
import { route } from "preact-router";
import { BrowseModInfo } from "../api/models/modinfo/browse";
import { invoke_proxy } from "../invoke";
import { useState } from "preact/hooks";
export interface ModParams {
    mod: BrowseModInfo;
}

export const Mod: FunctionalComponent<ModParams> = ({ mod }) => {
    const [installed, setInstalled] = useState(false);
    const [installing, setInstalling] = useState(false);

    const capText = (text: string, size: number) => {
        if (text.length > size) return `${text.substring(0, size - 3)}...`;

        return text;
    };

    const onDownload = async (ev: MouseEvent) => {
        ev.preventDefault();
        ev.stopPropagation();

        setInstalling(true);

        await invoke_proxy("install_mod", { modId: mod.id });
        // window.open(await invoke_proxy("install_mod", { modId: mod.id }));

        setInstalling(false);
        setInstalled(!installed);
    };

    return (
        <div className="mod-tile" onClick={() => route(`/mod/${mod.id}`)}>
            <img src={mod.background} className="image" />

            <div className="info">
                <p className="title">{capText(mod.name, 22)}</p>

                <button type="button" className="action" onClick={onDownload}>
                    {
                        installing ? <i className="icon fa-solid fa-spinner fa-spin" /> :
                        installed ? <i className="icon fa-solid fa-trash-can" /> :
                        <i className="icon fa-solid fa-circle-down" />
                    }
                </button>
            </div>
        </div>
    );
};
