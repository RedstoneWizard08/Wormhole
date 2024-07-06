import { event } from "@tauri-apps/api";

type Method = "Create" | "Read" | "Update" | "Delete";

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

const mapMethod = (method: Method) => {
    switch (method) {
        case "Create":
            return "PUT";
        case "Delete":
            return "DELETE";
        case "Read":
            return "GET";
        case "Update":
            return "POST";
    }
};

// Huge thanks to @darkylmnx: https://gist.github.com/tjmehta/9204891?permalink_comment_id=3527084#gistcomment-3527084
const serializeQuery = (initialObj: any) => {
    const reducer =
        (obj: any, parentPrefix: string | null = null) =>
        (prev: string[], key: string) => {
            const val = obj[key];
            key = encodeURIComponent(key);
            const prefix = parentPrefix ? `${parentPrefix}[${key}]` : key;

            if (val == null || typeof val === "function") {
                prev.push(`${prefix}=`);
                return prev;
            }

            if (["number", "boolean", "string"].includes(typeof val)) {
                prev.push(`${prefix}=${encodeURIComponent(val)}`);
                return prev;
            }

            prev.push(Object.keys(val).reduce(reducer(val, prefix), []).join("&"));
            return prev;
        };

    return Object.keys(initialObj).reduce(reducer(initialObj), []).join("&");
};

const responseQueue: Record<string, (data: unknown) => void> = {};

export const setupTauri = () => {
    event.listen<TauriOutput<unknown>>("plugin:rpc-rs:transport:resp", ({ payload: data }) => {
        responseQueue[data.id]?.(data.result);
    });
};

const __rpc_call = async <T, O>(
    routePrefix: string,
    method: Method,
    command: string,
    data: T
): Promise<O> => {
    if (!("__TAURI__" in window)) {
        if (method === "Read") {
            return await fetch(`${routePrefix}/${command}?${serializeQuery(data)}`, {
                method: "GET",
            }).then((v) => v.json());
        }

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


export async function __rpc_call_mods_Read(data: null): Promise<Mod[]> {
return await __rpc_call("/rpc", "Read", "__rpc_call_mods_Read", data);
}

export async function __rpc_call_mods_Create(data: ModCreation): Promise<bigint | string> {
return await __rpc_call("/rpc", "Create", "__rpc_call_mods_Create", data);
}

export async function __rpc_call_mods_Update(data: null): Promise<string> {
return await __rpc_call("/rpc", "Update", "__rpc_call_mods_Update", data);
}

export async function __rpc_call_mods_Delete(data: null): Promise<string> {
return await __rpc_call("/rpc", "Delete", "__rpc_call_mods_Delete", data);
}

const __rpc_module_mods = {
    create: __rpc_call_mods_Create,
    read: __rpc_call_mods_Read,
    update: __rpc_call_mods_Update,
    delete: __rpc_call_mods_Delete,
};


export async function __rpc_call_version_Read(data: null): Promise<string> {
return await __rpc_call("/rpc", "Read", "__rpc_call_version_Read", data);
}

export async function __rpc_call_version_Create(data: null): Promise<string> {
return await __rpc_call("/rpc", "Create", "__rpc_call_version_Create", data);
}

export async function __rpc_call_version_Update(data: null): Promise<string> {
return await __rpc_call("/rpc", "Update", "__rpc_call_version_Update", data);
}

export async function __rpc_call_version_Delete(data: null): Promise<string> {
return await __rpc_call("/rpc", "Delete", "__rpc_call_version_Delete", data);
}

const __rpc_module_version = {
    create: __rpc_call_version_Create,
    read: __rpc_call_version_Read,
    update: __rpc_call_version_Update,
    delete: __rpc_call_version_Delete,
};


export type Mod = { id: number; mod: string; version: string | null; name: string; file: string; size: number; hash: string | null; installed_files: string; sourceId: number; instanceId: number }

export type ModCreation = { mod: string; name: string; file: string; size: number; installed_files: string; sourceId: number; instanceId: number }

export const RPC = {
mods: __rpc_module_mods,
version: __rpc_module_version,
};

export default RPC;