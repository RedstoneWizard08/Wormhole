import { Route, Router } from "preact-router";
import { Home } from "./routes/Home";
import { InstallProgressWrapper } from "./routes/InstallProgressWrapper";

export const App = () => {
    return (
        <Router>
            <Route path="/" component={Home} />
            <Route path="/install" component={InstallProgressWrapper} />
        </Router>
    );
};
