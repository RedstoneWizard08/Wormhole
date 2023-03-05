import "./App.scss";
import { Route, Router } from "preact-router";
import { Header } from "./components/Header";
import { Home } from "./routes/Home";
import { InstallProgressWrapper } from "./routes/InstallProgressWrapper";
import { Instances } from "./routes/Instances";
import { Browse } from "./routes/mods/Browse";
import { FullMod } from "./routes/mods/FullMod";
import { Manage } from "./routes/mods/Manage";
import { InstanceInfo, KSPGame } from "./api/instance";
import { JSX } from "preact/jsx-runtime";

export const DEV_Instances: InstanceInfo[] = [
    {
        id: 0,
        name: "KSP2 Default Instance",
        game: KSPGame.KSP2,
        install_path:
            "/home/steam/.steam/root/steamapps/common/Kerbal Space Program 2",
        mods: [],
    },
];

export const createWrapper = (element: JSX.Element) => {
    const _wrapped = () => {
        return <>{element}</>;
    };

    return _wrapped;
};

export const App = () => {
    return (
        <div className="app-container">
            <Header />

            <div className="main">
                <Router>
                    <Route path="/" component={Home} />
                    <Route
                        path="/instances"
                        component={createWrapper(
                            <Instances instances={DEV_Instances} />
                        )}
                    />
                    <Route path="/mods" component={Browse} />
                    <Route path="/manage" component={Manage} />
                    <Route path="/install" component={InstallProgressWrapper} />
                    <Route path="/mod/:mod" component={FullMod} />
                </Router>
            </div>
        </div>
    );
};
