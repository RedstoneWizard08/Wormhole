import "./App.scss";
import { Router } from "preact-router";
import { Header } from "./components/Header";
import AsyncRoute from "preact-async-route";

export const App = () => {
    return (
        <div className="app-container">
            <Header />

            <div className="main">
                <Router>
                    <AsyncRoute
                        path="/"
                        getComponent={() =>
                            import("./routes/Home").then((m) => m.Home)
                        }
                    />

                    <AsyncRoute
                        path="/instances"
                        getComponent={() =>
                            import("./routes/Instances").then(
                                (m) => m.Instances
                            )
                        }
                    />

                    <AsyncRoute
                        path="/mods"
                        getComponent={() =>
                            import("./routes/mods/Browse").then((m) => m.Browse)
                        }
                    />

                    <AsyncRoute
                        path="/manage"
                        getComponent={() =>
                            import("./routes/mods/Manage").then((m) => m.Manage)
                        }
                    />

                    <AsyncRoute
                        path="/install"
                        getComponent={() =>
                            import("./routes/InstallProgress").then(
                                (m) => m.InstallProgress
                            )
                        }
                    />

                    <AsyncRoute
                        path="/mod/:mod"
                        getComponent={() =>
                            import("./routes/mods/FullMod").then(
                                (m) => m.FullMod
                            )
                        }
                    />

                    <AsyncRoute
                        path="/instance/:instance"
                        getComponent={() =>
                            import("./routes/Instance").then((m) => m.Instance)
                        }
                    />
                </Router>
            </div>
        </div>
    );
};
