import React from "react";
import ReactDOM from "react-dom/client";
import { App } from "./App";
import init from "rsw-hello";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <WasmWrapper />
  </React.StrictMode>
);

function WasmWrapper() {
  const [hasWasm, setHasWasm] = React.useState(false);
  React.useEffect(() => {
    init().then(() => {
      setHasWasm(true);
    });
  }, []);
  return hasWasm ? <App /> : <div>Loading...</div>;
}
