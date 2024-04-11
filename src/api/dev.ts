import { mockIPC } from "@tauri-apps/api/mocks";
import { SpaceDockAPI } from "./SpaceDock";
import { download } from "./mocks/download";
import ksp1_logo from "../assets/ksp-square.png";
import ksp2_logo from "../assets/ksp2-square.png";
import mc_logo from "../assets/minecraft.svg";
import ksp1_banner from "../assets/ksp.png";
import ksp2_banner from "../assets/ksp2.png";
import mc_banner from "../assets/minecraft_banner.jpg";
import type { Instance, PluginInfo } from "./bindings";

export const repeat = <T>(arr: T[], n: number): T[] => {
    const final: T[] = [];

    for (let i = 0; i < n; i++) {
        final.push(...arr);
    }

    return final;
};

export const DEV_Instances: Instance[] = [
    {
        id: 0,
        name: "KSP2 Default Instance",
        game_id: 22407,
        install_dir: "/home/user/.steam/root/steamapps/common/Kerbal Space Program 2",
        cache_dir: "",
        created: BigInt(new Date().getUTCMilliseconds()),
        data_dir: "",
        description: "",
        updated: BigInt(new Date().getUTCMilliseconds()),
    },

    {
        id: 1,
        name: "KSP1 Default Instance",
        game_id: 4102,
        install_dir: "/home/user/.steam/root/steamapps/common/Kerbal Space Program",
        cache_dir: "",
        created: BigInt(new Date().getUTCMilliseconds()),
        data_dir: "",
        description: "",
        updated: BigInt(new Date().getUTCMilliseconds()),
    },

    {
        id: 2,
        name: "Minecraft Default Instance",
        game_id: 432,
        install_dir: "/home/user/.minecraft",
        cache_dir: "",
        created: BigInt(new Date().getUTCMilliseconds()),
        data_dir: "",
        description: "",
        updated: BigInt(new Date().getUTCMilliseconds()),
    },
];

export const DEV_Plugins: PluginInfo[] = [
    {
        id: "KSP",
        display_name: "Kerbal Space Program",
        icon_url: ksp1_logo,
        banner_url: ksp1_banner,
        fallback_dir: "GameData",
        game: 3102,
        resolvers: ["SpaceDock", "Ckan"],
    },

    {
        id: "KSP2",
        display_name: "Kerbal Space Program 2",
        icon_url: ksp2_logo,
        banner_url: ksp2_banner,
        fallback_dir: "BepInEx/plugins",
        game: 22407,
        resolvers: ["SpaceDock", "Ckan"],
    },

    {
        id: "MC",
        display_name: "Minecraft",
        icon_url: mc_logo,
        banner_url: mc_banner,
        fallback_dir: "mods",
        game: 432,
        resolvers: ["CurseForge", "Modrinth"],
    },
];

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
