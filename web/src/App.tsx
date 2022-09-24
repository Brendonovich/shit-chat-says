import { createEffect, For, Show } from "solid-js";
import { rspc } from "./rspc";

const App = () => {
  const testQuery = rspc.createQuery(["allMessages"])

  createEffect(() => {
    console.log(testQuery.data)
  })

  return (
    <Show when={testQuery.data} fallback={null}>
      <For each={testQuery.data!}>
        {(message) => (
          <div class="">
            <span>{message.sender}</span>: <span>{message.content}</span>
          </div>
        )}
      </For>
    </Show>
  );
};

export default App;
