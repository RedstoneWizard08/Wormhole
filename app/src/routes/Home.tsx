import "./Home.scss";
import banner from "../assets/background_banner.png";
import { invoke_proxy } from "../invoke";
import { route } from "preact-router";

export const Home = () => {
    const install = async (action: "BepInEx" | "Doorstop" | "Uninstall") => {
        if (action == "BepInEx") {
            localStorage.setItem("kind", "BepInEx");
            localStorage.setItem("action", "Install");
        } else if (action == "Doorstop") {
            localStorage.setItem("kind", "Doorstop");
            localStorage.setItem("action", "Install");
        } else {
            const isBepInEx = await invoke_proxy("get_install_type");

            localStorage.setItem("kind", isBepInEx ? "BepInEx" : "Doorstop");
            localStorage.setItem("action", "Uninstall");
        }

        route("/install", true);
    };

    return (
        <div class="home-container">
            <img src={banner} alt="Space Warp Logo" class="logo" />

            <br />

            <h1 class="title">Welcome to the Space Warp Installer</h1>

            <div class="actions">
                <button
                    class="action"
                    id="bepinex"
                    onClick={() => install("BepInEx")}>
                    Install with BepInEx
                </button>

                <button
                    class="action"
                    id="doorstop"
                    onClick={() => install("Doorstop")}>
                    Install standalone
                </button>

                <button
                    class="action"
                    id="uninstall"
                    onClick={() => install("Uninstall")}>
                    Uninstall
                </button>
            </div>

            <button
                class="action"
                id="launch"
                onClick={() => invoke_proxy("launch")}>
                Launch
            </button>
        </div>
    );
};
