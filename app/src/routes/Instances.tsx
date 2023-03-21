import "./Instances.scss";
import { InstanceInfo } from "../api/instance";
import { Instance } from "../components/Instance";
import { useEffect, useState } from "preact/hooks";
import { invoke_proxy } from "../invoke";

export const Instances = () => {
    const [adding, setAdding] = useState(false);
    const [instances, setInstances] = useState<InstanceInfo[]>([]);

    const refreshInstances = async () => {
        const data = await invoke_proxy("get_instances", undefined);

        setInstances(data);
    };

    useEffect(() => {
        refreshInstances().then((r) => r);
    }, []);

    const addInstance = async () => {
        // await invoke_proxy("add_instance", {});
        setAdding(false);
    };

    return (
        <>
            {adding ? (
                <div className="add-modal-background">
                    <div className="add-modal">
                        <div className="modal-header">
                            <span className="title">
                                Add Instance
                            </span>

                            <i className="fa-solid fa-times close" onClick={addInstance} />
                        </div>
                    </div>
                </div>
            ) : (
                <></>
            )}

            <div className="instances-wrapper">
                <button
                    className="add-instance-button"
                    onClick={() => setAdding(!adding)}>
                    <i className="fa-solid fa-plus" />
                </button>

                <div className="instances-container">
                    {Array.isArray(instances) &&
                        instances.map((info) => (
                            <Instance data={info} key={info.name} />
                        ))}
                </div>
            </div>
        </>
    );
};
