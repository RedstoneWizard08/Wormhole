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

        let output = data;

        if ("Ok" in data) {
            output = data.Ok;
        }

        if ("Err" in data) {
            output = data.Err;
        }

        if ("Some" in data) {
            output = data.Some;
        }

        if ("None" in data || data == "None") {
            output = null;
        }

        console.log(`Tauri invoke: ${cmd}\n`, args, output);

        return output;
    });
};
