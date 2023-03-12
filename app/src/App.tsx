import "./App.scss";
import {Route, Router} from "preact-router";
import {Header} from "./components/Header";
import {Home} from "./routes/Home";
import {InstallProgress} from "./routes/InstallProgress";
import {Instances} from "./routes/Instances";
import {Browse} from "./routes/mods/Browse";
import {FullMod} from "./routes/mods/FullMod";
import {Manage} from "./routes/mods/Manage";
import {Instance} from "./routes/Instance";

export const App = () => {
    return (
        <div className="app-container">
            <Header />

            <div className="main">
                <Router>
                    <Route path="/" component={Home} />
                    <Route path="/instances" component={Instances} />

                    <Route path="/mods" component={Browse} />
                    <Route path="/manage" component={Manage} />
                    <Route path="/install" component={InstallProgress} />

                    <Route path="/mod/:mod" component={FullMod} />
                    <Route path="/instance/:instance" component={Instance} />
                </Router>
            </div>
        </div>
    );
};
