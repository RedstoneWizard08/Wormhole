import { event } from "@tauri-apps/api";

export type Method = "Create" | "Read" | "Update" | "Delete";
export type Result<T, E> = __Ok__<T> | __Err__<E>;
export type Option<T> = __Some__<T> | __None__;

type __Ok__<T> = { Ok: T };
type __Err__<E> = { Err: E };
type __Some__<T> = { Some: T };
type __None__ = { None: null };

interface TauriInput<T> {
    command: string;
    method: Method;
    data: T;
    id: string;
}

interface TauriOutput<T> {
    command: string;
    method: Method;
    result: T;
    id: string;
}

interface TauriInvokeInput<T> {
    command: string;
    data: T;
    id: string;
}

interface TauriInvokeOutput<T> {
    command: string;
    result: T;
    id: string;
}

const mapMethod = (method: Method) => {
    switch (method) {
        case "Create":
            return "PUT";
        case "Delete":
            return "DELETE";
        case "Read":
            return "POST";
        case "Update":
            return "PATCH";
    }
};

const responseQueue: Record<string, (data: unknown) => void> = {};

const __rpc_call = async <T, O>(
    routePrefix: string,
    method: Method,
    command: string,
    data: T
): Promise<O> => {
    if (!("__TAURI__" in window)) {
        return await fetch(`${routePrefix}/${command}`, {
            method: mapMethod(method),
            body: JSON.stringify(data),
        }).then((v) => v.json());
    }

    const id = crypto.randomUUID();

    const input: TauriInput<T> = {
        command,
        id,
        method,
        data,
    };

    const promise = new Promise<O>((res, _rej) => {
        responseQueue[input.id] = res as (data: unknown) => void;
    });

    event.emit("plugin:rpc-rs:transport", input);

    return promise;
};

const __rpc_invoke = async <T, O>(routePrefix: string, command: string, data: T): Promise<O> => {
    if (!("__TAURI__" in window)) {
        return await fetch(`${routePrefix}/_invoke/${command}`, {
            method: "POST",
            body: JSON.stringify(data),
        }).then((v) => v.json());
    }

    const id = crypto.randomUUID();

    const input: TauriInvokeInput<T> = {
        command,
        id,
        data,
    };

    const promise = new Promise<O>((res, _rej) => {
        responseQueue[input.id] = res as (data: unknown) => void;
    });

    event.emit("plugin:rpc-rs:transport:invoker", input);

    return promise;
};

export const setupTauri = () => {
    if (!("__TAURI__" in window)) return;

    event.listen<TauriOutput<unknown>>("plugin:rpc-rs:transport:resp", ({ payload: data }) => {
        responseQueue[data.id]?.(data.result);
    });

    event.listen<TauriInvokeOutput<unknown>>(
        "plugin:rpc-rs:transport:invoker:resp",
        ({ payload: data }) => {
            responseQueue[data.id]?.(data.result);
        }
    );
};
export const unwrap = <T>(res: Result<T, any> | Option<T>): T => {
    if (res === null || "None" in res) {
        throw new ReferenceError("Tried to unwrap a 'None' value!");
        // biome-ignore lint/style/noUselessElse: IT'S NOT REAL
    } else if ("Some" in res) {
        return res.Some;
        // biome-ignore lint/style/noUselessElse: IT'S NOT REAL
    } else {
        return (res as __Ok__<T>).Ok;
    }
};


export async function __rpc_call_instances_Read(data: null = null): Promise<Instance[]> {
return await __rpc_call("/rpc", "Read", "instances", data);
}

export async function __rpc_call_instances_Create(data: InstanceCreation[]): Promise<Result<bigint, string>> {
return await __rpc_call("/rpc", "Create", "instances", data);
}

export async function __rpc_call_instances_Update(data: null = null): Promise<string> {
return await __rpc_call("/rpc", "Update", "instances", data);
}

export async function __rpc_call_instances_Delete(data: number[]): Promise<Result<bigint, string>> {
return await __rpc_call("/rpc", "Delete", "instances", data);
}

const __rpc_module_instances = {
    create: __rpc_call_instances_Create,
    read: __rpc_call_instances_Read,
    update: __rpc_call_instances_Update,
    delete: __rpc_call_instances_Delete,
};


export async function __rpc_call_mod_Delete(data: number): Promise<Result<InstalledMod, string>> {
return await __rpc_call("/rpc", "Delete", "mod", data);
}

export async function __rpc_call_mod_Update(data: InstalledModUpdate): Promise<Result<InstalledMod, string>> {
return await __rpc_call("/rpc", "Update", "mod", data);
}

export async function __rpc_call_mod_Read(data: number): Promise<Option<InstalledMod>> {
return await __rpc_call("/rpc", "Read", "mod", data);
}

export async function __rpc_call_mod_Create(data: InstalledModCreation): Promise<Result<bigint, string>> {
return await __rpc_call("/rpc", "Create", "mod", data);
}

const __rpc_module_mod = {
    create: __rpc_call_mod_Create,
    read: __rpc_call_mod_Read,
    update: __rpc_call_mod_Update,
    delete: __rpc_call_mod_Delete,
};


export async function __rpc_call_instance_Create(data: InstanceCreation): Promise<Result<bigint, string>> {
return await __rpc_call("/rpc", "Create", "instance", data);
}

export async function __rpc_call_instance_Read(data: number): Promise<Option<Instance>> {
return await __rpc_call("/rpc", "Read", "instance", data);
}

export async function __rpc_call_instance_Update(data: InstanceUpdate): Promise<Result<Instance, string>> {
return await __rpc_call("/rpc", "Update", "instance", data);
}

export async function __rpc_call_instance_Delete(data: number): Promise<Result<Instance, string>> {
return await __rpc_call("/rpc", "Delete", "instance", data);
}

const __rpc_module_instance = {
    create: __rpc_call_instance_Create,
    read: __rpc_call_instance_Read,
    update: __rpc_call_instance_Update,
    delete: __rpc_call_instance_Delete,
};


export async function __rpc_call_mods_Create(data: InstalledModCreation[]): Promise<Result<bigint, string>> {
return await __rpc_call("/rpc", "Create", "mods", data);
}

export async function __rpc_call_mods_Delete(data: number[]): Promise<Result<bigint, string>> {
return await __rpc_call("/rpc", "Delete", "mods", data);
}

export async function __rpc_call_mods_Read(data: number): Promise<InstalledMod[]> {
return await __rpc_call("/rpc", "Read", "mods", data);
}

export async function __rpc_call_mods_Update(data: null = null): Promise<string> {
return await __rpc_call("/rpc", "Update", "mods", data);
}

const __rpc_module_mods = {
    create: __rpc_call_mods_Create,
    read: __rpc_call_mods_Read,
    update: __rpc_call_mods_Update,
    delete: __rpc_call_mods_Delete,
};


const __rpc_invokers = {

};

export type InstalledMod = { id: number; fileSize: number; updated: string; iconUrl: string | null; instanceId: number; source: string; projectId: string; projectName: string; projectDesc: string | null; projectVersion: string | null }

export type InstalledModCreation = { fileSize: number; instanceId: number; source: string; projectId: string; projectName: string }

export type InstalledModUpdate = { id: number; fileSize?: number | null; updated?: string | null; iconUrl?: string | null; instanceId?: number | null; source?: string | null; projectId?: string | null; projectName?: string | null; projectDesc?: string | null; projectVersion?: string | null }

export type Instance = { id: number; name: string; plugin: string; description: string; updated: string; iconUrl: string | null; bannerUrl: string | null; extraData: string }

export type InstanceCreation = { name: string; plugin: string; description: string; extraData: string }

export type InstanceUpdate = { id: number; name?: string | null; plugin?: string | null; description?: string | null; updated?: string | null; iconUrl?: string | null; bannerUrl?: string | null; extraData?: string | null }

export const RPC = {
instances: __rpc_module_instances,
mod: __rpc_module_mod,
instance: __rpc_module_instance,
mods: __rpc_module_mods,
invoke: __rpc_invokers,
};

export default RPC;