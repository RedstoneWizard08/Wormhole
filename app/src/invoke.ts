import { invoke } from "@tauri-apps/api";
import { InstanceInfo } from "./api/instance";
import { BrowseResult } from "./api/models/browse";
import { FullModInfo } from "./api/models/modinfo/full";

export interface InstanceArgs {
    instanceId: number;
}

export interface ModArgs {
    modId: number;
    gameId: number;
}

export interface ModsArgs {
    gameId: number;
    page: number;
    count: number;
}

export interface QueryData {
    query: string;
    modName: string;
}

export interface GameArgs {
    gameId: number;
}

export interface InstanceUpdateArgs {
    instanceId: number;
    description: string;
}

export interface InvokeFunction {
    install_spacewarp: [undefined, string];
    uninstall_spacewarp: [undefined, string];

    get_install_dir: [GameArgs, string];
    get_install_type: [undefined, string];

    launch: [GameArgs, undefined];

    get_instances: [undefined, InstanceInfo[]];
    get_instance_info: [InstanceArgs, InstanceInfo];

    get_mod: [ModArgs, FullModInfo];
    get_mod_download: [ModArgs, string];
    get_mods: [ModsArgs, BrowseResult];
    get_distance: [QueryData, undefined];

    install_mod: [ModArgs, undefined];
    backend_boot: [undefined, undefined];
    read_mod_json: [undefined, undefined];

    update_description: [InstanceUpdateArgs, undefined];
}

export const invoke_proxy = async <K extends keyof InvokeFunction>(
    action: K,
    args: InvokeFunction[K][0]
): Promise<InvokeFunction[K][1]> => {
    try {
        return (await invoke(action, { ...args })) as any;
    } catch (e) {
        return e as any;
    }
};
