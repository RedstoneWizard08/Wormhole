import "./InstallProgress.scss";
import banner from "../assets/background_banner.png";
import { useEffect, useState } from "preact/hooks";
import { invoke_proxy } from "../invoke";
import { route } from "preact-router";
import { listen } from "@tauri-apps/api/event";

let _listening = false;

export const InstallProgress = () => {
    const [status, setStatus] = useState({
        percent: "0%",
        message: "Waiting for start button...",
    });

    useEffect(() => {
        if (!_listening) {
            listen("download_progress", (ev) => {
                setStatus({
                    percent: ((100 * (ev.payload as any).received) / (ev.payload as any).total) + "%",
                    message: status.message,
                });
            });

            _listening = true;
        }
    });

    const [installed, setInstalled] = useState(false);

    const getInstallDir = async (): Promise<Error | null> => {
        setStatus({ percent: "0%", message: "Finding KSP2 location..." });

        const _dir = await invoke_proxy("get_install_dir");

        if (/(?:(:?[A-Z]:(?:\\|\/).+)|(:?\/.+))/gm.test(_dir)) {
            return null;
        }

        return new Error(
            `Could not find the KSP2 install directory! More details: ${_dir}`
        );
    };

    const doInstall = async (): Promise<Error | null> => {
        setStatus({ percent: "50%", message: "Installing SpaceWarp..." });

        const res = await invoke_proxy("download_bepinex");

        if (res == "Success") {
            return null;
        }

        return new Error(`Could not install SpaceWarp! More details: ${res}`);
    };

    const beginSetup = async () => {
        let err = await getInstallDir();

        if (err) {
            setStatus({ ...status, message: err.message });
            return;
        }

        await doInstall();

        setStatus({ percent: "100%", message: "Done!" });
        setInstalled(true);
    };

    const goBack = () => {
        route("/");
    };

    return (
        <div className="install-progress-container">
            <img src={banner} alt="Space Warp Logo" className="logo" />

            <br />

            <h1 className="title">
                Installing SpaceWarp...
            </h1>

            <p className="progress">
                {status.percent} - {status.message}
            </p>

            <button
                className="action"
                id="start"
                onClick={!installed ? beginSetup : goBack}>
                {!installed ? "Start" : "Back"}
            </button>
        </div>
    );
};
