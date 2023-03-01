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

export const downloadBepInEx = async () => {
    const url =
        "/_dev/BepInEx_x64_5.4.21.0.zip";

    const resp = await axios.get<Blob>(url, {
        onDownloadProgress: (ev) => {
            emit("download_progress_bepinex", {
                current: ev.loaded,
                total: ev.total,
                progress: ev.progress,
            });
        },

        responseType: "blob",
    });

    const fileName = url.split("/").pop()!;
    
    const dataUri = await toDataURI(resp.data);

    const a = document.createElement("a");

    a.href = dataUri;
    a.download = fileName;

    document.body.appendChild(a);
    
    a.click();
    a.remove();
};
