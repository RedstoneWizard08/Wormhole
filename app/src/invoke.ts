import { invoke } from "@tauri-apps/api";
import { InstanceInfo } from "./api/instance";

export interface DownloadArgs {
    kspGivenPath: string;
}

export interface InstanceArgs {
    instanceId: number;
}

export interface InvokeFunction {
    download_doorstop: [DownloadArgs, string];
    download_bepinex: [DownloadArgs, string];

    uninstall_doorstop: [undefined, string];
    uninstall_bepinex: [undefined, string];

    get_install_dir: [undefined, string];
    get_install_type: [undefined, string];

    launch: [undefined, undefined];

    get_instances: [undefined, InstanceInfo[]];
    get_instance_info: [InstanceArgs, InstanceInfo];
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
