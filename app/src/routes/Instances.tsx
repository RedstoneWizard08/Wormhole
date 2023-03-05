import "./Instances.scss";
import { InstanceInfo } from "../api/instance";
import { Instance } from "../components/Instance";
import { useState } from "preact/hooks";
import { invoke_proxy } from "../invoke";

export const Instances = () => {
    const [instances, setInstances] = useState<InstanceInfo[]>([]);

    const refreshInstances = async () => {
        const data = await invoke_proxy("get_instances");

        setInstances(data);
    };

    refreshInstances();

    return (
        <div className="instances-wrapper">
            <div className="instances-container">
                {instances.map((info) => (
                    <Instance data={info} key={info.name} />
                ))}
            </div>
        </div>
    );
};
