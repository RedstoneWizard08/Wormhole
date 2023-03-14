import "./Home.scss";
import banner from "../assets/background_banner.png";
import { route } from "preact-router";

export const Home = () => {
    const doInstall = () => {
        route("/install", true);
    };

    return (
        <div className="home-container">
            <div class="container">
                <img src={banner} alt="Space Warp Logo" className="logo" />
            </div>

            <br />

            <h1 className="title">Welcome to Wormhole</h1>

            <button type="button" className="action" onClick={doInstall}>
                Install SpaceWarp
            </button>
        </div>
    );
};
