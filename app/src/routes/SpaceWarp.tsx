import "./SpaceWarp.scss";
import banner from "../assets/background_banner.png";
import { route } from "preact-router";

export const SpaceWarp = () => {
    const doInstall = () => {
        route("/install", true);
    };

    return (
        <div className="spacewarp-container">
            <div class="container">
                <img src={banner} alt="Space Warp Logo" className="logo" />
            </div>

            <button type="button" className="action" onClick={doInstall}>
                Install SpaceWarp
            </button>
        </div>
    );
};
