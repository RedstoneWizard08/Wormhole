import {
    listen as realListen,
    type EventName,
    type EventCallback,
    type UnlistenFn,
} from "@tauri-apps/api/event";

export const formatBytes = (n: number) => {
    const k = n > 0 ? Math.floor(Math.log2(n) / 10) : 0;
    const rank = `${k > 0 ? "KMGTPE"[k - 1] : ""}b`;
    const count = (n / 1024 ** k).toFixed(1);
    return `${count} ${rank}`;
};

export const isChildOf = (node: Node, ancestor: HTMLElement) => {
    let child: Node | null = node;

    while (child !== null) {
        if (child === ancestor) return true;

        child = child.parentNode;
    }

    return false;
};
