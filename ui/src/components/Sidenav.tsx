import { Component } from "solid-js";
import { NavLink } from "@solidjs/router";

const Sidenav: Component = () => {
  return (
    <div class="bg-dark-400 text-light-400 p-2 overflow-auto scrollbar-hide">
      <div class="flex justify-center items-center h-12 w-12 mb-2">
        <h2 class="font-bold font-mono">
          VIS
          <br />
          ORM
        </h2>
      </div>
      <nav>
        <ul class="flex flex-col gap-y-4 justify-center items-center">
          <li class="border border-gray-500 rounded-lg p-2">
            <NavLink href="/" class="block bg-gray-500 rounded-md w-8 h-8" />
          </li>
          <li class="rounded-lg p-2">
            <NavLink href="/" class="block bg-gray-500 rounded-md w-8 h-8" />
          </li>
          <li class="rounded-lg p-2">
            <NavLink href="/" class="block bg-gray-500 rounded-md w-8 h-8" />
          </li>
          <li class="rounded-lg p-2">
            <NavLink href="/" class="block bg-gray-500 rounded-md w-8 h-8" />
          </li>
          <li class="rounded-lg p-2">
            <NavLink href="/" class="block bg-gray-500 rounded-md w-8 h-8" />
          </li>
        </ul>
      </nav>
    </div>
  );
};

export default Sidenav;
