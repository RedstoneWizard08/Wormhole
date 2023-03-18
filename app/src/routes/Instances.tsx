import "./Instances.scss";
import { InstanceInfo } from "../api/instance";
import { Instance } from "../components/Instance";
import { useEffect, useState } from "preact/hooks";
import { invoke_proxy } from "../invoke";

export const Instances = () => {
    const [instances, setInstances] = useState<InstanceInfo[]>([]);

    const refreshInstances = async () => {
        const data = await invoke_proxy("get_instances", undefined);
        
        setInstances(data);
    };

    useEffect(() => {
        refreshInstances().then((r) => r);
    }, []);

    return (
        <div className="instances-wrapper">
            <button className="add-instance-button">
                <i className="fa-solid fa-plus" />
            </button>

            <div className="instances-container">
                {Array.isArray(instances) &&
                    instances.map((info) => (
                        <Instance data={info} key={info.name} />
                    ))}
            </div>
        </div>
    );
};
