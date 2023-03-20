import axios from "axios";
import { emit } from "@tauri-apps/api/event";

export const toDataURI = (blob: Blob) => {
    return new Promise<string>((resolve, reject) => {
        const reader = new FileReader();

        reader.onload = () => resolve(reader.result as string);
        reader.onerror = () => reject(reader.error);
        reader.onabort = () => reject(new Error("Read aborted!"));

        reader.readAsDataURL(blob);
    });
};

export const save = (url: string, name: string) => {
    const a = document.createElement("a");

    a.href = url;
    a.download = name;

    document.body.appendChild(a);

    a.click();
    a.remove();
};

export const downloadBepInEx = async () => {
    const url = "/_dev/BepInEx_x64_5.4.21.0.zip";
    const url2 = "/_dev/spacewarp-release-1.0.1.zip";

    const resp = await axios.get<Blob>(url, {
        onDownloadProgress: (ev) => {
            emit("download_progress", {
                received: ev.loaded,
                total: ev.total! * 2,
            });
        },

        responseType: "blob",
    });

    const resp2 = await axios.get<Blob>(url2, {
        onDownloadProgress: (ev) => {
            emit("download_progress", {
                received: ev.total! + ev.loaded,
                total: ev.total! * 2,
            });
        },

        responseType: "blob",
    });

    const fileName = url.split("/").pop()!;
    const fileName2 = url2.split("/").pop()!;

    const dataUri = await toDataURI(resp.data);
    const dataUri2 = await toDataURI(resp2.data);

    save(dataUri, fileName);
    save(dataUri2, fileName2);
};
