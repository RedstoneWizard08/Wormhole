import { FunctionalComponent } from "preact";
import { InstanceInfo } from "../api/instance";
import { Instance } from "../components/Instance";
import "./Instances.scss";

export interface InstancesProps {
    instances: InstanceInfo[];
}

export const Instances: FunctionalComponent<InstancesProps> = ({ instances }) => {
    return (
        <div className="instances-container">
            {instances.map((info) => {
                <Instance data={info} />
            })}
        </div>
    );
};
