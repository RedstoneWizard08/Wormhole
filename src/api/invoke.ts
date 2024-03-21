import type { InstanceInfo } from "./instance";
import type { BrowseResult } from "./models/browse";
import type { FullModInfo } from "./models/modinfo/full";
import type { Plugin } from "./models/plugin";

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

export interface ModIntegrity {
    name: string;
    date_installed: string;
    size: number;
    install_path: string;
}

export interface ModsIntegrity {
    mods: ModIntegrity[];
}

export interface AddInstanceArgs {
    gameId: number;
    name: string;
    installPath: string;
}

export interface GetInstances {
    gameId: number;
}

export interface InvokeFunction {
    install_spacewarp: [undefined, string];
    uninstall_spacewarp: [undefined, string];

    get_install_dir: [GameArgs, string];

    launch: [InstanceArgs, undefined];

    get_instances: [GetInstances, InstanceInfo[]];
    get_instance_info: [InstanceArgs, InstanceInfo];

    get_mod: [ModArgs, FullModInfo];
    get_mod_download: [ModArgs, string];
    get_mods: [ModsArgs, BrowseResult];
    get_distance: [QueryData, undefined];

    add_instance: [AddInstanceArgs, undefined];
    install_mod: [ModArgs & InstanceArgs, undefined];
    backend_boot: [undefined, undefined];
    read_mod_json: [undefined, ModsIntegrity];

    update_description: [InstanceUpdateArgs, undefined];
    get_active_instance: [GameArgs, InstanceInfo | undefined];
    set_active_instance: [InstanceArgs, undefined];

    delete_instance: [InstanceArgs, undefined];
    get_plugins: [undefined, Record<number, Plugin>];
}

export const invoke_proxy = async <K extends keyof InvokeFunction>(
    action: K,
    args: InvokeFunction[K][0]
): Promise<InvokeFunction[K][1]> => {
    try {
        const invoke = (await import("@tauri-apps/api")).invoke;

        return (await invoke(action, { ...args })) as any;
    } catch (e) {
        return e as any;
    }
};
