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
                <><img src={ksp2logo} className="logo" alt={"background"} />
                </>
            ) : (
                <img src={ksp1logo} className="logo" alt={"background"} />
            )}
            <p className="name">{data.name}</p>
            <div className="buttons">
                <button
                    type="button"
                    className="action"
                    id="launch-button"
                    onClick={(e) => {
                        e.stopPropagation(); // stop the click event from bubbling up to the container
                        console.log("test");
                    }}
                ><i className="icon fa-solid fa-caret-up" /> {/* TODO: find better icon */}
                </button>
                <button
                    type="button"
                    className="action"
                    id="delete-button"
                    onClick={(e) => {
                        e.stopPropagation(); // stop the click event from bubbling up to the container
                        console.log("test");
                    }}
                ><i className="icon fa-solid fa-trash-can" />
                </button>
            </div>
        </div>
    );
};
