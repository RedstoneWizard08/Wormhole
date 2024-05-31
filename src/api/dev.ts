import { emit } from "@tauri-apps/api/event";
import { mockIPC } from "@tauri-apps/api/mocks";

export let socket: WebSocket | null = null;
export const globalEventBus = new EventTarget();

export const createMockAPI = () => {
    mockIPC(async (cmd, args) => {
        console.log(`Tauri invoke: ${cmd}\n`, args);

        if (
            cmd === "tauri" &&
            "message" in args &&
            "cmd" in (args.message as any) &&
            (args.message as any).cmd === "emit"
        ) {
            return globalEventBus.dispatchEvent(
                new CustomEvent((args.message as any).event, {
                    detail: (args.message as any).payload,
                })
            );
        }

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

        if ("None" in data || data === "None") {
            output = null;
        }

        console.log(`Invoke result (${cmd}):`, output);

        return output;
    });

    const proto = window.location.protocol === "https:" ? "wss" : "ws";
    const host = window.location.host;
    const port = window.location.port === "" ? "" : `:${window.location.port}`;
    const url = `${proto}://${host}${port}/_tauri/events`;

    console.log(`Connecting to events socket at ${url}...`);

    socket = new WebSocket(url);

    socket.addEventListener("open", () => {
        console.log("Events socket connected!");
    });

    socket.addEventListener("message", async (ev) => {
        const data = JSON.parse(ev.data);
        const key = data.event;
        const payload = data.data;

        console.log(`Dispatching event ${key} with payload:`, payload);

        await emit(key, payload);
    });
};
