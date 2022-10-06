import { Component, createSignal, For, Index } from "solid-js";
import {
  createSolidTable,
  ColumnDef,
  getCoreRowModel,
  flexRender,
} from "@tanstack/solid-table";
import CreateTable from "./components/CreateTable";
import Sidenav from "./components/Sidenav";

type Field = {
  name: string;
  type: string;
  nullable: boolean;
  default: string;
  unique: boolean;
  references: string[];
};

const defaultData: Field[] = [
  {
    name: "id",
    type: "int",
    nullable: false,
    default: "auto_increment",
    unique: true,
    references: ["", "Self"],
  },
];

const types = ["INT", "SERIAL", "TEXT", "VARCHAR"];

const defaultColumns: ColumnDef<Field>[] = [
  {
    header: "Action",
    cell: (row) => (
      <button
        class="bg-gray-500 px-2 rounded text-sm"
        onClick={() => console.log(row)}
      >
        helo
      </button>
    ),
  },
  {
    accessorKey: "name",
    header: "Field Name",
    cell: (row) => (
      <input
        value={row.getValue<string>()}
        class="bg-gray-500 rounded text-white text-sm focus:(ring-blue-500 border-blue-500) block placeholder-gray-400 px-2 outline-none"
      />
    ),
  },
  {
    accessorKey: "type",
    header: "Type",
    cell: (row) => (
      <select class="bg-dark-600 text-white text-xs focus:(ring-blue-500 border-blue-500) block placeholder-gray-400 px-2 outline-none">
        <For each={types}>
          {(type) =>
            row.getValue<string>().toUpperCase() == type ? (
              <option selected value={type}>
                {type}
              </option>
            ) : (
              <option value={type}>{type}</option>
            )
          }
        </For>
      </select>
    ),
  },
  {
    accessorKey: "default",
    header: "Default Value",
    cell: (row) => row.getValue(),
  },
  {
    accessorKey: "nullable",
    header: "Nullable",
    cell: (row) => (
      <input
        type="checkbox"
        name="nullable"
        class="block w-4 h-4 bg-gray-700 border-gray-600 rounded border border-gray-600 focus:(ring-3 ring-blue-600) ring-offset-gray-800"
        checked={row.getValue<boolean>()}
      />
    ),
  },
  {
    accessorKey: "unique",
    header: "Unique",
    cell: (row) => (
      <input
        type="checkbox"
        name="nullable"
        class="block w-4 h-4 bg-gray-700 border-gray-600 rounded border border-gray-600 focus:(ring-3 ring-blue-600) ring-offset-gray-800"
        checked={row.getValue<boolean>()}
      />
    ),
  },
  {
    accessorKey: "references",
    header: "References",
    cell: (row) => (
      <select class="bg-dark-600 text-white text-xs focus:(ring-blue-500 border-blue-500) block placeholder-gray-400 px-2 outline-none">
        <For each={row.getValue<string[]>()}>
          {(reference) =>
            reference === "" ? (
              <option selected value={reference}>
                [null]
              </option>
            ) : (
              <option value={reference}>{reference}</option>
            )
          }
        </For>
      </select>
    ),
  },
];

const App: Component = () => {
  const [data, setData] = createSignal(defaultData);
  const table = createSolidTable({
    get data() {
      return data();
    },
    columns: defaultColumns,
    getCoreRowModel: getCoreRowModel(),
  });
  return (
    <div class="flex h-screen max-h-screen w-full max-w-full">
      <Sidenav />
      <main class="flex-1 overflow-auto scrollbar-hide">
        <CreateTable />
        <table class="w-full">
          <thead>
            <For each={table.getHeaderGroups()}>
              {(headerGroup) => (
                <tr>
                  <For each={headerGroup.headers}>
                    {(header) => (
                      <th class="border border-gray-500">
                        {header.isPlaceholder
                          ? null
                          : flexRender(
                              header.column.columnDef.header,
                              header.getContext()
                            )}
                      </th>
                    )}
                  </For>
                </tr>
              )}
            </For>
          </thead>
          <tbody>
            <Index each={table.getRowModel().rows}>
              {(row) => (
                <tr>
                  <For each={row().getVisibleCells()}>
                    {(cell) => (
                      <td class="border border-gray-500">
                        {flexRender(
                          cell.column.columnDef.cell,
                          cell.getContext()
                        )}
                      </td>
                    )}
                  </For>
                </tr>
              )}
            </Index>
          </tbody>
        </table>
      </main>
    </div>
  );
};

export default App;
