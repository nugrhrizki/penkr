import type { Component, ParentProps } from "solid-js";

const Group: Component<ParentProps> = (props) => {
  return <div class="mb-4">{props.children}</div>;
};

export default Group;
