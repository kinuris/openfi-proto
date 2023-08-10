import { $, component$, useSignal, useStylesScoped$, useVisibleTask$ } from "@builder.io/qwik";
import type { DataPlanData, PlanCollectionData, PlanData } from "~/util/types";

export default component$(() => {
    useStylesScoped$(`
        div.plan-container {
            width: 100%;
        }

        div.time-plan-container {
            width: 60%;
            display: flex;
            justify-content: space-between;
            border-bottom: 2px solid orange;
            padding: 0.2em;
            margin: auto;
        }

        p {
            text-align: center;
            margin: 0 1em;
        }

        p button {
            width: 25px;
            height: 25px;
        }
    `);

    const plans = useSignal<PlanCollectionData>();

    useVisibleTask$(async () => {
        const res = await fetch(`http://${import.meta.env.VITE_SITE_URL}/get-plans`);
        const rawPlans = await res.json();

        const dataPlans = (rawPlans["data_plans"] as any[]).map(plan => ({
            id: plan["id"],
            creditCost: plan["credit_cost"],
            megabytesGiven: plan["megabytes_given"],
            name: plan["name"]
        } as DataPlanData));

        const timePlans = (rawPlans["time_plans"] as any[]).map(plan => ({
            id: plan["id"],
            creditCost: plan["credit_cost"],
            secondsGiven: plan["seconds_given"],
            name: plan["name"]
        } as PlanData));

        plans.value = {
            dataPlans,
            timePlans
        } as PlanCollectionData;
    });

    const onUseCreditHandler = $(async (id: number) => {
        const res = await fetch(`http://${import.meta.env.VITE_SITE_URL}/spend/TIME/${id}`, { method: "POST" });
        
        if (res.status >= 400) {
            alert("Error: " + res.statusText);
            return;
        }

        alert("Success");
    });

    return (
        <div class="plan-container">
            <div class="time-plan-container">
                <h3>Credit Cost</h3>
                <h3>Time Given</h3>
            </div>
            {plans.value ? plans.value.timePlans.map((plan) =>
            (<div class="time-plan-container" key={plan.id}>
                <p>{plan.creditCost} PHP</p>
                <p>{plan.name} <button onClick$={() => onUseCreditHandler(plan.id)}>+</button></p>
            </div>))
                : <div class="plan-container">
                    <p>...Loading</p>
                </div>}
        </div>
    );
});