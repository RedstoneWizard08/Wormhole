import { invoke } from "@tauri-apps/api";
import { InstanceInfo } from "./api/instance";
import { BrowseResult } from "./api/models/browse";
import { FullModInfo } from "./api/models/modinfo/full";

export interface DownloadArgs {
    kspGivenPath: string;
}

export interface InstanceArgs {
    instanceId: number;
}

export interface ModArgs {
    modId: number;
}

export interface ModsArgs {
    gameId: number;
    page: number;
    count: number;
}

export interface InvokeFunction {
    download_bepinex: [DownloadArgs, string];
    uninstall_bepinex: [undefined, string];

    get_install_dir: [undefined, string];
    get_install_type: [undefined, string];

    launch: [undefined, undefined];

    get_instances: [undefined, InstanceInfo[]];
    get_instance_info: [InstanceArgs, InstanceInfo];

    get_mod: [ModArgs, FullModInfo];
    get_mod_download: [ModArgs, string];
    get_mods: [ModsArgs, BrowseResult];

    install_mod: [ModArgs, undefined];
}

export const invoke_proxy = async <K extends keyof InvokeFunction>(
    action: K,
    args?: InvokeFunction[K][0]
): Promise<InvokeFunction[K][1]> => {
    try {
        return (await invoke(action, { ...args })) as any;
    } catch (e) {
        return e as any;
    }
};
