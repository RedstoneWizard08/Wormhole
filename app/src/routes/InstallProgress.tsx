import "./InstallProgress.scss";
import banner from "../assets/background_banner.png";
import { useState } from "preact/hooks";
import { invoke_proxy } from "../invoke";
import { FunctionalComponent } from "preact";
import { route } from "preact-router";

export enum InstallKind {
    // eslint-disable-next-line no-unused-vars
    BepInEx,

    // eslint-disable-next-line no-unused-vars
    Doorstop,
}

export enum InstallAction {
    // eslint-disable-next-line no-unused-vars
    Install,

    // eslint-disable-next-line no-unused-vars
    Uninstall,
}

export interface InstallProgressProps {
    kind: InstallKind;
    action: InstallAction;
}

export const InstallProgress: FunctionalComponent<InstallProgressProps> = ({
    kind,
    action,
}) => {
    const [status, setStatus] = useState({
        percent: "0%",
        message: "Waiting for start button...",
    });

    const [installed, setInstalled] = useState(false);

    const getInstallDir = async (): Promise<Error | null> => {
        setStatus({ percent: "0%", message: "Finding KSP2 location..." });

        const _dir = await invoke_proxy("get_install_dir");

        if (/[A-Z]:(?:\\|\/).+/gm.test(_dir)) {
            return null;
        }

        return new Error(
            `Could not find the KSP2 install directory! More details: ${_dir}`
        );
    };

    const doInstall = async (): Promise<Error | null> => {
        setStatus({ percent: "50%", message: "Installing SpaceWarp..." });

        const res = await invoke_proxy(
            kind == InstallKind.BepInEx
                ? "download_bepinex"
                : "download_doorstop"
        );

        if (res == "Success") {
            return null;
        }

        return new Error(`Could not install SpaceWarp! More details: ${res}`);
    };

    const doUninstall = async (): Promise<Error | null> => {
        setStatus({ percent: "50%", message: "Uninstalling SpaceWarp..." });

        const res = await invoke_proxy(
            kind == InstallKind.BepInEx
                ? "uninstall_bepinex"
                : "uninstall_doorstop"
        );

        if (res == "Success") {
            return null;
        }

        return new Error(`Could not uninstall SpaceWarp! More details: ${res}`);
    };

    const beginSetup = async () => {
        let err = await getInstallDir();

        if (err) {
            setStatus({ ...status, message: err.message });
            return;
        }

        if (action == InstallAction.Install) err = await doInstall();
        else err = await doUninstall();

        if (err) {
            setStatus({ ...status, message: err.message });
            return;
        }

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
                Installing SpaceWarp (
                {kind == InstallKind.Doorstop ? "Standalone" : "BepInEx"})...
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
