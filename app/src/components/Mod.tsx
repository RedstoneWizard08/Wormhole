import "./Mod.scss";
import { FunctionalComponent } from "preact";
import { route } from "preact-router";
import { BrowseModInfo } from "../api/models/modinfo/browse";
export interface ModParams {
    mod: BrowseModInfo;
}

export const Mod: FunctionalComponent<ModParams> = ({ mod }) => {
    const capText = (text: string, size: number) => {
        if (text.length > size) return text.substring(0, size - 3) + "...";

        return text;
    };

    const onDownload = (ev: MouseEvent) => {
        ev.preventDefault();
        ev.stopPropagation();
    };

    return (
        <div className="mod-tile" onClick={() => route(`/mod/${mod.id}`)}>
            <img src={mod.background} className="image" />

            <div className="info">
                <p className="title">{capText(mod.name, 22)}</p>

                <button type="button" className="action" onClick={onDownload}>
                    <i className="icon fa-solid fa-circle-down" />
                </button>
            </div>
        </div>
    );
};
