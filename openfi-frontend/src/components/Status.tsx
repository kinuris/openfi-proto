import { $, component$, useSignal, useStylesScoped$ } from "@builder.io/qwik";
import { secondsToFormattedTime } from "~/util/time";
import type { ClientData } from "~/util/types";

export default component$(({ data }: { data: ClientData }) => {

    useStylesScoped$(`
        button {
            padding: 0.3em 1em;
            border-radius: 10px;
            font-size: 1.5em;
            margin-top: 1em;
        }

        div p {
            text-align: center;
        }

        h1 {
            margin-bottom: 0.5em;
            font-size: 2em;
        }
    `);

    const disabled = useSignal(false);

    // TODO: Add spinners while awaiting response
    const onPauseHandler = $(async () => {
        disabled.value = true;
        const res = await fetch(`http://${import.meta.env.VITE_SITE_URL}/pause`, { method: 'post' });
        if (res.status === 200) {
            alert("Successfully paused connection.");
        }
        disabled.value = false;
    });

    const onConnectHandler = $(async () => {
        disabled.value = true;
        const res = await fetch(`http://${import.meta.env.VITE_SITE_URL}/request-access`, { method: 'post' });
        if (res.status === 200) {
            alert("Successfully connected.")
        }
        disabled.value = false;
    });

    return (
        <>
            <h1>User Data</h1>
            <div>
                <p>Time: {secondsToFormattedTime(data.remainingSeconds)}</p>
                <p>Credits: {data.credits} PHP</p>
                <p>Status: {data.active ? "Connected" : "Not Connected"}</p>
            </div>
            {data.remainingSeconds > 0 ? (data.active ? <button disabled={disabled.value} onClick$={onPauseHandler}>Pause</button> : <button disabled={disabled.value} onClick$={onConnectHandler}>Connect</button>) : <button>Insert Coins</button>}
        </>
    );
});