import { mockIPC } from "@tauri-apps/api/mocks";
import { type InstanceInfo, KSPGame } from "./instance";
import { SpaceDockAPI } from "./SpaceDock";
import type { InvokeFunction, ModsIntegrity } from "./invoke";
import { download } from "./mocks/download";
import type { Plugin } from "./models/plugin";
import ksp1_logo from "../assets/ksp-square.png";
import ksp2_logo from "../assets/ksp2-square.png";
import mc_logo from "../assets/minecraft.svg";
import ksp1_banner from "../assets/ksp.png";
import ksp2_banner from "../assets/ksp2.png";
import mc_banner from "../assets/minecraft_banner.jpg";

export const repeat = <T>(arr: T[], n: number): T[] => {
    const final: T[] = [];

    for (let i = 0; i < n; i++) {
        final.push(...arr);
    }

    return final;
};

export const DEV_Instances: InstanceInfo[] = [
    {
        id: 0,
        name: "KSP2 Default Instance",
        game: KSPGame.KSP2,
        install_path: "/home/user/.steam/root/steamapps/common/Kerbal Space Program 2",
        mods: [],
    },

    {
        id: 1,
        name: "KSP1 Default Instance",
        game: KSPGame.KSP1,
        install_path: "/home/user/.steam/root/steamapps/common/Kerbal Space Program",
        mods: [],
    },

    {
        id: 2,
        name: "Minecraft Default Instance",
        game: 432,
        install_path: "/home/user/.minecraft",
        mods: [],
    },
];

export const DEV_Plugins: Record<number, Plugin> = {
    3102: {
        id: 3102,
        name: "KSP1",
        display: "Kerbal Space Program",
        icon: ksp1_logo,
        banner: ksp1_banner,

        caps: {
            mods: true,
            multiple_instances: true,
        },

        settings: {},
    },

    22407: {
        id: 22407,
        name: "KSP2",
        display: "Kerbal Space Program 2",
        icon: ksp2_logo,
        banner: ksp2_banner,

        caps: {
            mods: true,
            multiple_instances: true,
        },

        settings: {},
    },

    432: {
        id: 432,
        name: "Minecraft",
        display: "Minecraft",
        icon: mc_logo,
        banner: mc_banner,

        caps: {
            mods: true,
            multiple_instances: true,
        },

        settings: {},
    },
};

export const createMockAPI = () => {
    mockIPC((async <K extends keyof InvokeFunction>(
        cmd: K,
        args: InvokeFunction[K][0]
    ): Promise<InvokeFunction[K][1]> => {
        switch (cmd) {
            case "install_spacewarp":
                await download();
                return "Success";

            case "uninstall_spacewarp":
                return "Success";

            case "get_install_dir":
                return "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Kerbal Space Program 2";

            case "launch":
                return undefined;

            case "get_instances":
                return DEV_Instances.filter((v) => v.game == (args as any | undefined)?.gameId);

            case "get_instance_info":
                const id = (args as InvokeFunction["get_instance_info"][0]).instanceId;

                return DEV_Instances.find((v) => v.id == id);

            case "get_mods":
                const fargs = args as InvokeFunction["get_mods"][0];

                return await new SpaceDockAPI().getModsForGame(
                    fargs.gameId,
                    fargs.page,
                    fargs.count
                );

            case "get_mod":
                return await new SpaceDockAPI().getMod(
                    (args as InvokeFunction["get_mod"][0]).modId
                );

            case "get_mod_download":
                return await new SpaceDockAPI().getModDownload(
                    (args as InvokeFunction["get_mod_download"][0]).modId
                );

            case "install_mod":
                return await new SpaceDockAPI().getModDownload(
                    (args as InvokeFunction["install_mod"][0]).modId
                );

            case "read_mod_json":
                const res: ModsIntegrity = {
                    mods: [],
                };

                return res;
            
            case "get_plugins":
                return DEV_Plugins;
        }
    }) as any);
};
