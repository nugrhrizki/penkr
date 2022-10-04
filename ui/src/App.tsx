import { Component } from "solid-js";
import CreateTable from "./components/CreateTable";
import Sidenav from "./components/Sidenav";

const App: Component = () => {
  return (
    <div class="flex h-screen max-h-screen w-full max-w-full">
      <Sidenav />
      <main class="flex-1 overflow-auto scrollbar-hide">
        <CreateTable />
      </main>
    </div>
  );
};

export default App;
