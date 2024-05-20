// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

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

export const download = async () => {
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
