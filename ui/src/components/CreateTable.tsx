import { Component, Index } from "solid-js";
import { createForm } from "@felte/solid";
import { IconPlus, IconTrash } from "./Icons";
import Group from "./forms/Group";
import Label from "./forms/Label";

type Field = {
  name: string;
  type: string;
  nullable: boolean;
  default: string;
  unique: boolean;
};

type PrimaryKey = {
  name: string;
  type: string;
};

type Table = {
  name: string;
  primary_key: PrimaryKey;
  fields: Field[];
};

const CreateTable: Component = () => {
  const { form, data, addField, unsetField } = createForm<Table>({
    initialValues: {
      name: "",
      primary_key: {
        name: "id",
      },
      fields: [],
    },
    onSubmit: (values) => {
      console.log(values);
    },
  });

  const fields = () => data("fields");

  function addNewField(index: number) {
    return function () {
      addField(
        "fields",
        {
          name: "",
          type: "",
          default: "",
          nullable: false,
          unique: false,
        },
        index
      );
    };
  }

  function removeField(index: number) {
    return function () {
      unsetField(`fields.${index}`);
    };
  }

  return (
    <form use:form class="my-4">
      <Group>
        <Label for="name">Table Name</Label>
        <div class="flex gap-x-4">
          <input
            name="name"
            id="name"
            placeholder="name"
            class="bg-gray-700 border border-gray-600 text-white text-sm rounded-lg focus:(ring-blue-500 border-blue-500) block w-full p-2 placeholder-gray-400 outline-none"
            required
          />
          <button
            type="submit"
            class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
          >
            Create
          </button>
        </div>
      </Group>
      <Group>
        <Label for="primary_key.name">Primary Key</Label>
        <div class="flex gap-x-4">
          <input
            name="primary_key.name"
            id="primary_key.name"
            placeholder="id"
            class="bg-gray-700 border border-gray-600 text-white text-sm rounded-lg focus:(ring-blue-500 border-blue-500) block w-full p-2 placeholder-gray-400 outline-none"
            required
          />
          <select
            name="primary_key.type"
            id="primary_key.type"
            class="bg-gray-700 border border-gray-600 text-white text-sm rounded-lg focus:(ring-blue-500 border-blue-500) block p-2 placeholder-gray-400 outline-none"
            required
          >
            <option selected value="uuid">
              uuid
            </option>
            <option value="bigserial">bigserial</option>
            <option value="serial">serial</option>
            <option value="smallserial">smallserial</option>
            <option value="integer">integer</option>
            <option value="bigint">bigint</option>
            <option value="smallint">smallint</option>
          </select>
        </div>
      </Group>
      <Group>
        <div
          onClick={addNewField(fields().length)}
          class="flex justify-center items-center w-min bg-blue-700 hover:bg-blue-800 text-white focus:(ring-4 outline-none ring-blue-300) rounded-lg text-xl p-2"
        >
          <IconPlus />
        </div>
      </Group>
      <Index each={fields()}>
        {(_, index) => (
          <Group>
            <div class="flex gap-x-4 items-center">
              <Group>
                <Label for={`fields.${index}.name`}>Name</Label>
                <input
                  name={`fields.${index}.name`}
                  id={`fields.${index}.name`}
                  placeholder="name"
                  class="bg-gray-700 border border-gray-600 text-white text-sm rounded-lg focus:(ring-blue-500 border-blue-500) block w-full p-2 placeholder-gray-400 outline-none"
                  required
                />
              </Group>
              <Group>
                <Label for={`fields.${index}.type`}>Type</Label>
                <select
                  name={`fields.${index}.type`}
                  id={`fields.${index}.type`}
                  class="bg-gray-700 border border-gray-600 text-white text-sm rounded-lg focus:(ring-blue-500 border-blue-500) block p-2 placeholder-gray-400 outline-none"
                  required
                >
                  <option value="" selected disabled>
                    [Choose One]
                  </option>
                  <option value="text">Text</option>
                  <option value="int">Int</option>
                  <option value="bool">Bool</option>
                  <option value="timestamp">Timestamp</option>
                  <option value="timestamptz">Timestamp with timezone</option>
                </select>
              </Group>
              <Group>
                <Label for={`fields.${index}.default`}>Default</Label>
                <input
                  name={`fields.${index}.default`}
                  id={`fields.${index}.default`}
                  placeholder="default"
                  class="bg-gray-700 border border-gray-600 text-white text-sm rounded-lg focus:(ring-blue-500 border-blue-500) block w-full p-2 placeholder-gray-400 outline-none"
                />
              </Group>
              <Group>
                <Label for={`field.${index}.nullable`}>Null</Label>
                <input
                  type="checkbox"
                  name={`fields.${index}.nullable`}
                  class="block w-4 h-4 bg-gray-700 border-gray-600 rounded border border-gray-600 focus:(ring-3 ring-blue-600) ring-offset-gray-800"
                />
              </Group>
              <Group>
                <Label for={`field.${index}.unique`}>Unique</Label>
                <input
                  type="checkbox"
                  name={`fields.${index}.unique`}
                  class="block w-4 h-4 bg-gray-700 border-gray-600 rounded border border-gray-600 focus:(ring-3 ring-blue-600) ring-offset-gray-800"
                />
              </Group>
              <button
                type="button"
                onClick={removeField(index)}
                class="flex justify-center items-center w-min text-white bg-blue-700 hover:bg-blue-800 focus:(ring-4 outline-none ring-blue-300) font-medium rounded-lg text-sm w-full p-2"
              >
                <IconTrash />
              </button>
              <button
                type="button"
                onClick={addNewField(index + 1)}
                class="flex justify-center items-center w-min bg-blue-700 hover:bg-blue-800 text-white focus:(ring-4 outline-none ring-blue-300) rounded-lg text-xl p-2"
              >
                <IconPlus />
              </button>
            </div>
          </Group>
        )}
      </Index>
    </form>
  );
};

export default CreateTable;
