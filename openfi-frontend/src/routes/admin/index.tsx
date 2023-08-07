import { $, component$, useSignal, useStyles$, useStylesScoped$, useVisibleTask$ } from "@builder.io/qwik";
import { DHMToSeconds, secondsToFormattedTime } from "~/util/time";
import type { CodeData } from "~/util/types";

export default component$(() => {
  useStyles$(`
  .container {
    width: min(400px, 100vw);
    height: calc(100% - 20px);
    background-color: white;
    border-left: 2px solid orange;
    border-right: 2px solid orange;
  }
`);

  useStylesScoped$(`
    .container {
      display: flex;
      flex-direction: column;
      align-items: center;
      padding: 10px;
    }

    form {
      display: flex;
      flex-direction: column;
      width: 80%;
    }

    form input {
      padding: 0.5em;
      font-size: 1em;
    }

    ul li {
      margin: 1em;
    }

    h1 {
      margin: 1em;
      font-size: 2em;
    }

    h3 {
      margin: 0 0 1em 0;
    }
  `);

  const days = useSignal("");
  const hours = useSignal("");
  const minutes = useSignal("");
  const codes = useSignal<CodeData[]>([]);

  const onGenCodeHandler = $(async () => {
    const seconds = DHMToSeconds(Number(days.value), Number(hours.value), Number(minutes.value));

    if (seconds === 0) {
      alert("Code will have Zero (0) Seconds of time");
      return;
    }

    const res = await fetch(`http://${import.meta.env.VITE_SITE_URL}/gen-code/TIME/${seconds}`, { method: 'POST' });

    if (res.status >= 400) {
      alert("Code Generation Failed")
      return;
    }

    const code = await res.text();
    const codeComplete = {
      code,
      kind: 'TIME',
      units: seconds
    } as CodeData

    // TODO: use track() in useVisibleTask$ instead of manually appeding to codes
    codes.value = [...codes.value, codeComplete];
  });

  useVisibleTask$(async () => {
    const res = await fetch(`http://${import.meta.env.VITE_SITE_URL}/get-codes`, { method: 'POST' });
    const codesJson = await res.json() as CodeData[]; 
    // NOTE: These 'CodeData' objects actually have an extra 'id' property
    // SUGGESTION: consider deleting this property

    codes.value = [...codes.value, ...codesJson]
  });

  return (
    <section class="container">
      <h1>Admin Page</h1>
      <h3>Generate Time Code</h3>
      <form onSubmit$={onGenCodeHandler} preventdefault:submit>
        <input type="number" name="days" min={0} placeholder="Enter Days" bind:value={days} />
        <input type="number" name="hours" min={0} placeholder="Enter Hours" bind:value={hours} />
        <input type="number" name="mins" min={0} placeholder="Enter Minutes" bind:value={minutes} />
        <input type="submit" value="Generate Code" />
      </form>
      <ul>
        {codes.value.map(code => (
          <li key={code.code}>
            <p>Code: {code.code}</p>
            <p>Units: {secondsToFormattedTime(code.units)}</p>
            <p>Kind: {code.kind}</p>
          </li>
        ))}
      </ul>
    </section>
  )
})