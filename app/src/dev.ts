import { mockIPC } from "@tauri-apps/api/mocks";
import { downloadBepInEx } from "./mocks/download";

export const createMockAPI = () => {
    mockIPC(async (cmd, args) => {
        switch (cmd) {
            case "download_doorstop":
                return "Success";

            case "download_bepinex":
                await downloadBepInEx();
                return "Success";

            case "uninstall_doorstop":
                return "Success";

            case "uninstall_bepinex":
                return "Success";

            case "get_install_dir":
                return "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Kerbal Space Program 2";

            case "get_install_type":
                return true;

            case "launch":
                return undefined;
        }
    });
};
