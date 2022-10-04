import { Component, JSX, splitProps, ParentProps } from "solid-js";

const Label: Component<
  ParentProps & JSX.LabelHTMLAttributes<HTMLLabelElement>
> = (props) => {
  const [local, others] = splitProps(props, ["class"]);
  return (
    <label
      class={
        "block mb-2 text-sm font-medium text-gray-300" + local.class
          ? " " + local.class
          : ""
      }
      {...others}
    >
      {props.children}
    </label>
  );
};

export default Label;
