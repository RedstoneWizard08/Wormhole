import { InstallAction, InstallKind, InstallProgress } from "./InstallProgress";

export const InstallProgressWrapper = () => {
    return (
        <InstallProgress
            kind={
                localStorage.getItem("kind") == "BepInEx"
                    ? InstallKind.BepInEx
                    : InstallKind.Doorstop
            }
            action={
                localStorage.getItem("action") == "Install"
                    ? InstallAction.Install
                    : InstallAction.Uninstall
            }
        />
    );
};
