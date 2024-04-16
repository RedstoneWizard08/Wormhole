import { mockIPC } from "@tauri-apps/api/mocks";

export const createMockAPI = () => {
    mockIPC(async (cmd, args) => {
        const data = await fetch("/_tauri/invoke", {
            method: "POST",
            body: JSON.stringify({
                cmd,
                data: args,
            }),
            headers: {
                "Content-Type": "application/json",
            },
        }).then((res) => res.json());

        if ("Ok" in data) {
            return data.Ok;
        }

        if ("Err" in data) {
            return data.Err;
        }

        if ("Some" in data) {
            return data.Some;
        }

        if ("None" in data || data == "None") {
            return null;
        }

        return data;
    });
};
