import "@fortawesome/fontawesome-free/scss/fontawesome.scss";
import "@fortawesome/fontawesome-free/scss/regular.scss";
import "@fortawesome/fontawesome-free/scss/solid.scss";
import "@fortawesome/fontawesome-free/scss/brands.scss";

import { render } from "preact";
import { App } from "./App";
import { createMockAPI } from "./dev";
import eruda from "eruda";
import { backend_boot } from "./boot";
import { LoadingPage } from "./components/LoadingPage";


const root = document.getElementById("root")!;

const main = async () => {
    render(<LoadingPage />, root);
    
    await backend_boot();
    
    if (import.meta.env.TAURI_WEB_DEV) {
        eruda.init();
        eruda.position({ x: 10, y: window.innerHeight - 45 });
    
        createMockAPI();
    }
    
    render(<App />, root);
};

main();
