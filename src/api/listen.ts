import type { UnlistenFn } from "@tauri-apps/api/event";

export interface DownloadProgress {
    total: number;
    received: number;
}

export interface DownloadComplete {
    size: number;
}

export interface ListenEventType {
    download_progress: DownloadProgress;
    download_complete: DownloadComplete;
}

export interface ListenEvent<T> {
    payload: T;
}

// eslint-disable-next-line no-unused-vars
export type ListenHandler<T> = (event: ListenEvent<T>) => void;

export const listen_proxy = async <K extends keyof ListenEventType>(
    action: K,
    handler: ListenHandler<ListenEventType[K]>
): Promise<UnlistenFn> => {
    const listen = (await import("@tauri-apps/api/event")).listen;

    return listen(action, handler);
};
