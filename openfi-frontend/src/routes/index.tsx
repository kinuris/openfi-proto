import { $, component$, useSignal, useStyles$, useStylesScoped$, useVisibleTask$ } from "@builder.io/qwik";
import Status from "~/components/Status";
import type { ClientData } from "~/util/types";

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
  section#status {
    color: white;
    background-color: var(--bg-color);
    height: 200px;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
  }

  form {
    padding: 1em;
    background-color: var(--bg-color);
    border-top: 1px solid orange;
    border-bottom: 1px solid orange;
  }

  form h1 {
    color: white;
    margin-bottom: 1em;
    text-align: center;
    font-size: 2.5em;
  }

  form input {
    display: block;
    margin: 5px auto;
    border-radius: 10px;
    color: var(--bg-color);
  }

  form input[type="text"] {
    font-size: 2em;
    width: 12ch;
    border: 1px solid orange;
    background-color: white;
    padding: 5px 10px;
    text-align: center;
  }

  form input[type="submit"] {
    padding: 5px 10px;
    width: 235.5px;
    font-size: 1.5em;
    color: orange;
    border: 1px solid orange;
    background-color: var(--bg-color);
    transition: all 0.2s;
  }

  form input[type="submit"]:hover {
    transform: scale(1.01);
  }

  form input[type="submit"]:active {
    transform: scale(0.99);
  }
  `);

  const code = useSignal('');
  const clientData = useSignal<ClientData>();

  useVisibleTask$(async () => {
    const statusStream = new EventSource(`http://${import.meta.env.VITE_SITE_URL}/data-stream`);

    function parseAssign(event: MessageEvent<any>) {
      const raw = JSON.parse(event.data);
      const data = {
        id: raw["id"],
        active: raw["active"],
        credits: raw["credits"],
        mac: raw["mac"],
        remainingSeconds: raw["remaining_seconds"]
      } as ClientData;

      clientData.value = data;
    }

    statusStream.addEventListener('message', parseAssign);
  }, { strategy: 'document-ready' });

  const onRedeemHandler = $(async () => {
    if (code.value.length != 10) {
      alert('Code entered must be 10 characters');

      return;
    }

    const res = await fetch(`http://${import.meta.env.VITE_SITE_URL}/redeem-code`, {
      method: "POST", body: JSON.stringify({
        code: code.value
      }),
      headers: {
        "Content-Type": "application/json"
      }
    });

    if (res.status >= 400) {
      alert("Failed");
      return;
    }

    alert("Success");
  });

  return (
    <section class="container">
      <section id="status">
        <h1>User Data</h1>
        {clientData.value ? <Status data={clientData.value} /> : <div>Loading...</div>}
      </section>

      <form onSubmit$={onRedeemHandler} preventdefault:submit>
        <h1>Redeem your time and data codes</h1>
        <input type="text" name="code" maxLength={10} placeholder="Enter Code" bind:value={code} />
        <input type="submit" value="Redeem" />
      </form>
    </section>
  )
})