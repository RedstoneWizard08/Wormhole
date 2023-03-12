import { FunctionalComponent } from "preact";
import { StateUpdater, useState } from "preact/hooks";
import "./Dropdown.scss";

export interface DropdownProps {
    val?: string;
    setVal?: StateUpdater<string>;

    valText?: string;
    setValText?: StateUpdater<string>;
}

export const Dropdown: FunctionalComponent<DropdownProps> = ({
    val,
    setVal,
    valText,
    setValText,
}) => {
    if (!val || !setVal)
        // eslint-disable-next-line react-hooks/rules-of-hooks
        [val, setVal] = useState("ksp1");

    if (!valText || !setValText)
        // eslint-disable-next-line react-hooks/rules-of-hooks
        [valText, setValText] = useState("KSP 1");

    const [shown, setShown] = useState(false);

    const makeOnSelected = (val: string, txt: string) => {
        return () => {
            setVal!(val);
            setValText!(txt);

            setShown(false);
        };
    };

    const onClick = () => {
        setShown(!shown);
    };

    return (
        <div className={`dropdown ${shown ? "active" : ""}`}>
            <div
                className={`selected ${shown ? "active" : ""}`}
                onClick={onClick}>
                {valText}
            </div>

            <div className={`items ${!shown ? "hide" : ""}`}>
                <div
                    className={val == "ksp1" ? "same" : ""}
                    onClick={makeOnSelected("ksp1", "KSP 1")}>
                    KSP 1
                </div>

                <div
                    className={val == "ksp2" ? "same" : ""}
                    onClick={makeOnSelected("ksp2", "KSP 2")}>
                    KSP 2
                </div>
            </div>
        </div>
    );
};
