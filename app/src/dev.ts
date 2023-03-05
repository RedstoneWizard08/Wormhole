import { mockIPC } from "@tauri-apps/api/mocks";
import { InstanceInfo, KSPGame } from "./api/instance";
import { downloadBepInEx } from "./mocks/download";

export const repeat = <T, >(arr: T[], n: number): T[] => {
    const final: T[] = [];

    for (let i = 0; i < n; i++) {
        final.push(...arr);
    }

    return final;
};

export const DEV_Instances: InstanceInfo[] = repeat([
    {
        id: 0,
        name: "KSP2 Default Instance",
        game: KSPGame.KSP2,
        install_path:
            "/home/steam/.steam/root/steamapps/common/Kerbal Space Program 2",
        mods: [],
    },
    {
        id: 1,
        name: "KSP1 Default Instance",
        game: KSPGame.KSP1,
        install_path:
            "/home/steam/.steam/root/steamapps/common/Kerbal Space Program",
        mods: [],
    },
], 1);

export const createMockAPI = () => {
    // eslint-disable-next-line no-unused-vars
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
            
            case "get_instances":
                return DEV_Instances;
            
            case "get_instance_info":
                const id = (args as any).instanceId;

                return DEV_Instances.find((v) => v.id == id);
        }
    });
};
