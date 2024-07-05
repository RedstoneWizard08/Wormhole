import {
    listen as realListen,
    type EventName,
    type EventCallback,
    type UnlistenFn,
} from "@tauri-apps/api/event";
import { globalEventBus } from "./dev";

export type __Result__<T, E> = T | E;

export const formatBytes = (n: number) => {
    const k = n > 0 ? Math.floor(Math.log2(n) / 10) : 0;
    const rank = `${k > 0 ? "KMGTPE"[k - 1] : ""}b`;
    const count = (n / 1024 ** k).toFixed(1);
    return `${count} ${rank}`;
};

export const unwrap = <T, E>(res: __Result__<T, E>): T => {
    return res as T;
};

export const listen = async <T>(
    event: EventName,
    handler: EventCallback<T>
): Promise<UnlistenFn> => {
    if (import.meta.env.TAURI_WEB_DEV) {
        const onEvent = (evt: Event) => {
            const ev = evt as CustomEvent;

            handler({ event, id: -1, payload: ev.detail, windowLabel: "" });
        };

        globalEventBus.addEventListener(event, onEvent);

        return () => {
            globalEventBus.removeEventListener(event, onEvent);
        };
    }
    return await realListen(event, handler);
};

export const isChildOf = (node: Node, ancestor: HTMLElement) => {
    let child: Node | null = node;

    while (child !== null) {
        if (child === ancestor) return true;

        child = child.parentNode;
    }

    return false;
};
