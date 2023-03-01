import { Route, Router } from "preact-router";
import { Header } from "./components/Header";
import { Home } from "./routes/Home";
import { InstallProgressWrapper } from "./routes/InstallProgressWrapper";
import { Instances } from "./routes/Instances";
import { Browse } from "./routes/mods/Browse";
import { FullMod } from "./routes/mods/FullMod";
import { Manage } from "./routes/mods/Manage";

export const App = () => {
    return (
        <div className="app-container">
            <Header />
            <Router>
                <Route path="/" component={Home} />
                <Route path="/instances" component={Instances} />
                <Route path="/mods" component={Browse} />
                <Route path="/manage" component={Manage} />
                <Route path="/install" component={InstallProgressWrapper} />
                <Route path="/mod/:mod" component={FullMod} />
            </Router>
        </div>
    );
};
