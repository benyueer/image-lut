import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import ImageView from "./components/image_view";
import {Router} from '@reach/router';
import DirSelect from "./pages/dirSelect";
import MainView from "./pages/mainView";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  // async function greet() {
  //   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  //   setGreetMsg(await invoke("greet", { name }));
  // }

  return (
    <Router style={{width: '100%', height: '100%'}}>
      <DirSelect path="/"></DirSelect>
      <MainView path="lut-view"/>
    </Router>
  );
}

export default App;
