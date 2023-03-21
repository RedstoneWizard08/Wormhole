import { FunctionalComponent } from "preact";
import { StateUpdater, useState } from "preact/hooks";
import "./Dropdown.scss";

export interface DropdownItem {
    id: string | number;
    text: string;
}

export interface DropdownProps {
    val?: string | number;
    setVal?: StateUpdater<string> | StateUpdater<number>;

    valText?: string;
    setValText?: StateUpdater<string>;

    left?: boolean;
    items: DropdownItem[];
}

export const Dropdown: FunctionalComponent<DropdownProps> = ({
    val,
    setVal,
    valText,
    setValText,
    left,
    items,
}) => {
    if (!val || !setVal)
        // eslint-disable-next-line react-hooks/rules-of-hooks
        [val, setVal] = useState("ksp1");

    if (!valText || !setValText)
        // eslint-disable-next-line react-hooks/rules-of-hooks
        [valText, setValText] = useState("KSP 1");

    const [shown, setShown] = useState(false);

    const makeOnSelected = (val: string | number, txt: string) => {
        return () => {
            setVal!(val as any);
            setValText!(txt);

            setShown(false);
        };
    };

    const onClick = () => {
        setShown(!shown);
    };

    return (
        <div
            className={`dropdown ${shown ? "active" : ""}`}
            style={{ justifySelf: left ? "start" : "end" }}>
            <div
                className={`selected ${shown ? "active" : ""}`}
                onClick={onClick}>
                {valText}
            </div>

            <div className={`items ${!shown ? "hide" : ""}`}>
                {items.map((item) => (
                    <div
                        className={val == item.id ? "same" : ""}
                        onClick={makeOnSelected(item.id, item.text)}
                        key={item.id}>
                        {item.text}
                    </div>
                ))}
            </div>
        </div>
    );
};
