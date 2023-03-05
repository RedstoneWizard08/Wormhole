import { FunctionalComponent } from "preact";
import { InstanceInfo } from "../api/instance";
import "./Instance.scss";

export interface InstanceProps {
    data: InstanceInfo;
}

export const Instance: FunctionalComponent<InstanceProps> = ({ data }) => {
    return (
        <div className="instance-container">
            {data.name}
        </div>
    );
};
