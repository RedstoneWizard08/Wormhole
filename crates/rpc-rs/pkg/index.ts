import { event } from "@tauri-apps/api";

export type Method = "Create" | "Read" | "Update" | "Delete";

export interface TauriInput<T> {
    command: string;
    method: Method;
    data: T;
    id: string;
}

export interface TauriOutput<T> {
    command: string;
    method: Method;
    result: T;
    id: string;
}

export type Result<T, E> = T | E;

export const mapMethod = (method: Method) => {
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
export const serializeQuery = (initialObj: any) => {
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

export const responseQueue: Record<string, (data: unknown) => void> = {};

export const setupTauri = () => {
    event.listen<TauriOutput<unknown>>("plugin:rpc-rs:transport:resp", ({ payload: data }) => {
        responseQueue[data.id]?.(data.result);
    });
};

export const request = async <T, O>(
    routePrefix: string,
    method: Method,
    command: string,
    data: T
): Promise<Result<O, string>> => {
    if (!("__TAURI__" in window)) {
        if (method === "Read") {
            return (await fetch(`${routePrefix}/${command}?${serializeQuery(data)}`, {
                method: "GET",
            }).then((v) => v.json())) as Result<O, string>;
        }

        return (await fetch(`${routePrefix}/${command}`, {
            method: mapMethod(method),
            body: JSON.stringify(data),
        }).then((v) => v.json())) as Result<O, string>;
    }

    const id = crypto.randomUUID();

    const input: TauriInput<T> = {
        command,
        id,
        method,
        data,
    };

    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    const promise = new Promise<Result<O, string>>((res, _rej) => {
        responseQueue[input.id] = res as (data: unknown) => void;
    });

    event.emit("plugin:rpc-rs:transport", input);

    return promise;
};
