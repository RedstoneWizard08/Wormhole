import { FunctionalComponent } from "preact";
import { route } from "preact-router";
import { InstanceInfo, KSPGame } from "../api/instance";
import ksp1logo from "../assets/ksp.png";
import ksp2logo from "../assets/ksp2.png";
import "./Instance.scss";

export interface InstanceProps {
    data: InstanceInfo;
}

export const Instance: FunctionalComponent<InstanceProps> = ({ data }) => {
    const clicked = () => {
        route(`/instance/${data.id}`);
    };

    return (
        <div className="instance-container" onClick={clicked}>
            {data.game == KSPGame.KSP2 ? (
                <img src={ksp2logo} className="logo" />
            ) : (
                <img src={ksp1logo} className="logo" />
            )}

            <p className="name">{data.name}</p>
        </div>
    );
};
