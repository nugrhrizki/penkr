import { JSX, Component, splitProps } from "solid-js";
import { createField } from "@felte/solid";

interface Props extends JSX.InputHTMLAttributes<HTMLInputElement> {
  name: string;
}

const Input: Component<Props> = (props) => {
  const [local, others] = splitProps(props, ["class", "name"]);
  const { field, onInput, onBlur } = createField(local.name);

  return (
    <input
      use:field
      onInput={(e) => onInput(e.currentTarget.innerText)}
      onBlur={onBlur}
      class={
        "bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" +
        local.class
          ? " " + local.class
          : ""
      }
      {...others}
    />
  );
};

export default Input;
