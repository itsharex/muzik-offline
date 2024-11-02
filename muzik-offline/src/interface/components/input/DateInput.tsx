import { FunctionComponent, useState } from 'react';
import "@styles/components/input/DateInput.scss";

type DateInputProps = {
    onChange: (date: Date) => void;
}

const reg = /^\d+$/;

const inputFields = [
  {
    name: "year",
    maxLength: 4,
    placeholder: "yyyy",
    className: "DateInput DateInput-year",
    value: "",
    separator: "/",
  },
  {
    name: "month",
    maxLength: 2,
    placeholder: "m",
    className: "DateInput",
    value: "",
    separator: "/",
  },
  {
    name: "day",
    maxLength: 2,
    placeholder: "dd",
    className: "DateInput",
    value: "",
    separator: ",",
  },
  {
    name: "hour",
    maxLength: 2,
    placeholder: "hh",
    className: "DateInput",
    value: "",
    separator: ":",
  },
  {
    name: "minute",
    maxLength: 2,
    placeholder: "m",
    className: "DateInput",
    value: "",
    separator: ":",
  },
  {
    name: "second",
    maxLength: 2,
    placeholder: "ss",
    className: "DateInput",
    value: "",
    separator: "",
  }
];

const DateInput: FunctionComponent<DateInputProps> = (props: DateInputProps) => {
  const [input, setInput] = useState<{
    name: string;
    maxLength: number;
    placeholder: string;
    className: string;
    value: string;
    separator: string;
  }[]>(inputFields);

  const keyboardEvent = (e: React.KeyboardEvent<HTMLInputElement>) => {
    const { name, value } = e.currentTarget;

    // if keypress is backspace and value.length === 1, then autofocus to the previous input
    if(e.key === "Backspace" && value.length === 1){
      // Update the input field with empty value
      setInput(input.map(field => (field.name === name ? { ...field, value: "" } : field)));
      return;
    }

    // if keypress is backspace and value === "", then autofocus to the previous input
    if(e.key === "Backspace" && value === "" && name === "month"){
      document.getElementsByName("year")[0].focus();
      return;
    } else if(e.key === "Backspace" && value === "" && name === "day"){
      document.getElementsByName("month")[0].focus();
      return;
    } else if(e.key === "Backspace" && value === "" && name === "hour"){
      document.getElementsByName("day")[0].focus();
      return;
    } else if(e.key === "Backspace" && value === "" && name === "minute"){
      document.getElementsByName("hour")[0].focus();
      return;
    } else if(e.key === "Backspace" && value === "" && name === "second"){
      document.getElementsByName("minute")[0].focus();
      return;
    } else if(e.key === "Backspace" && value === "" && name === "year"){
      return;
    }

    // if value has reached max length, then autofocus to the next input
    if(value.length === 4 && name === "year" && e.key !== "Backspace"){
      document.getElementsByName("month")[0].focus();
    } else if(value.length === 2 && name === "month" && e.key !== "Backspace"){
      document.getElementsByName("day")[0].focus();
    } else if(value.length === 2 && name === "day" && e.key !== "Backspace"){
      document.getElementsByName("hour")[0].focus();
    } else if(value.length === 2 && name === "hour" && e.key !== "Backspace"){
      document.getElementsByName("minute")[0].focus();
    } else if(value.length === 2 && name === "minute" && e.key !== "Backspace"){
      document.getElementsByName("second")[0].focus();
    } else if(value.length === 2 && name === "second" && e.key !== "Backspace"){
      return;
    }
  }

      const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const { name, value } = e.target;

        // validate input and check that input is not backspace
        if(name === "year" && !reg.test(value))return;
        if(name === "month" && !reg.test(value))return;
        if(name === "day" && !reg.test(value))return;
        if(name === "hour" && !reg.test(value))return;
        if(name === "minute" && !reg.test(value))return;
        if(name === "second" && !reg.test(value))return;

        // Update the input field
        setInput(input.map(field => (field.name === name ? { ...field, value } : field)));
        
        // Validate and combine the input.value parts if all are filled
        if (Object.values({ ...input, [name]: value }).every(part => part)) {
          props.onChange(new Date(
            Number.parseInt(input[0].value),
            Number.parseInt(input[1].value) - 1,  // JS months are 0-based
            Number.parseInt(input[2].value),
            Number.parseInt(input[3].value),
            Number.parseInt(input[4].value),
            Number.parseInt(input[5].value)
          ));
        }
      };
    
      return (
        <div className='DateInput-Container'>
          {input.map((field, index) => (
            <div key={index}>
              <input
                name={field.name}
                maxLength={field.maxLength}
                placeholder={field.placeholder}
                className={field.className}
                value={field.value}
                onChange={handleInputChange}
                onKeyDown={keyboardEvent}
              />
              {field.separator}
            </div>
          ))}
        </div>
      );
}

export default DateInput