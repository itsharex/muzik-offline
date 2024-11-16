import { motion } from 'framer-motion';
import { FunctionComponent } from 'react';
import "@styles/components/input/RadioComponent.scss";

type RadioComponentProps = {
    icon: () => JSX.Element | null;
    value: boolean;
    type: string;
    text: string;
    setviewableEl: (value: boolean, type: string) => void;
}

const RadioComponent: FunctionComponent<RadioComponentProps> = (props: RadioComponentProps) => {
    return (
        <motion.div 
            className={"radio_component " + (props.icon !== null ? "" : "radio_component_small")} whileHover={{scale: 1.03}} whileTap={{scale: 0.97}} 
            onClick={() => props.setviewableEl(!props.value, props.type)}>
                {props.icon !== null && <props.icon />}
                <h4>{props.text}</h4>
                <div className={"radio-display " + (props.value ? "selected" : "")}/>
        </motion.div>
    )
}

export default RadioComponent