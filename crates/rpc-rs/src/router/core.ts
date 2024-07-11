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

    return initialObj === null || initialObj === undefined
        ? ""
        : `?${Object.keys(initialObj).reduce(reducer(initialObj), []).join("&")}`;
};

const responseQueue: Record<string, (data: unknown) => void> = {};

const __rpc_call = async <T, O>(
    routePrefix: string,
    method: Method,
    command: string,
    data: T
): Promise<O> => {
    if (!("__TAURI__" in window)) {
        if (method === "Read") {
            return await fetch(`${routePrefix}/${command}${serializeQuery(data)}`, {
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

    event.listen<TauriOutput<unknown>>(
        "plugin:rpc-rs:transport:invoker:resp",
        ({ payload: data }) => {
            responseQueue[data.id]?.(data.result);
        }
    );
};
export const unwrap = <T>(res: Result<T, any> | Option<T>): T => {
    if ("None" in res) {
        throw new ReferenceError("Tried to unwrap a 'None' value!");
        // biome-ignore lint/style/noUselessElse: IT'S NOT REAL
    } else if ("Some" in res) {
        return res.Some;
        // biome-ignore lint/style/noUselessElse: IT'S NOT REAL
    } else {
        return (res as __Ok__<T>).Ok;
    }
};
