import "./Instances.scss";
import { InstanceInfo } from "../api/instance";
import { Instance } from "../components/Instance";
import { useState, useEffect } from "preact/hooks";
import { invoke_proxy } from "../invoke";

export const Instances = () => {
    const [instances, setInstances] = useState<InstanceInfo[]>([]);

    const refreshInstances = async () => {
        const data = await invoke_proxy("get_instances");
        setInstances(data);
    };

    useEffect(() => {
        refreshInstances().then(r => r);
    }, []);

    return (
        <div className="instances-wrapper">
            <button className="add-instance-button">Add Instance</button>
            <div className="instances-container">
                {Array.isArray(instances) && instances.map((info) => (
                    <Instance data={info} key={info.name} />
                ))}
            </div>
        </div>
    );

};
