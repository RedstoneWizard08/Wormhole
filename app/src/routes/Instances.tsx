import { invoke_proxy } from "../invoke";
import "./Instances.scss";

export const Instances = () => {
    return (
        <div className="instances-container">
            <button type="button" onClick={() => invoke_proxy("launch")}>
                Launch
            </button>
        </div>
    );
};
