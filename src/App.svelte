<script lang="ts">
    import DateComponent from "./lib/Date.svelte";
    import Route from "./lib/Route.svelte";
    import {
        deleteLastTicket,
        sendTicket,
        getAllTickets,
    } from "./lib/rust_commands";
    import { isTicketValid } from "./lib/rust_commands";
    let date: Date;
    let upStation: string = "";
    let downStation: string = "";
    let price: number = 0;
    let distance: number = 0;

    let validTicket: boolean = false;
    let ticket: string = "";
    let tickets: string[] = [];

    async function inputHandle() {
        validTicket = isTicketValid(
            ticket,
            upStation,
            downStation,
            date,
            price,
            distance,
        );
    }

    let sendTicketAndUpdate = async () => {
        if (
            !isTicketValid(
                ticket,
                upStation,
                downStation,
                date,
                price,
                distance,
            )
        )
            return;

        await sendTicket(ticket, upStation, downStation, date, price, distance);
        tickets = await getAllTickets(upStation, downStation);
        ticket = "";
        validTicket = false;
    };

    let removeHandle = async () => {
        await deleteLastTicket(downStation, upStation);
        tickets = await getAllTickets(downStation, upStation);
    };

    async function isEnterPressed(e: KeyboardEvent) {
        if (e.code == "Enter" || e.code == "NumpadEnter") sendTicketAndUpdate();
    }
</script>

<main>
    <div class="hero bg-base-200 min-h-screen">
        <div class="hero-content text-center">
            <div class="max-w-md">
                <DateComponent bind:date />
                <Route
                    bind:upStation
                    bind:downStation
                    bind:price
                    bind:distance
                    bind:tickets
                />

                {#if downStation}
                    <div>
                        <div class="row pt-10 join">
                            <input
                                class="input input-bordered w-full max-w-xs pa-10"
                                id="ticket-input"
                                placeholder="XXX-XXXXXX"
                                bind:value={ticket}
                                minlength="8"
                                maxlength="10"
                                on:input={inputHandle}
                                on:keypress={isEnterPressed}
                                class:field-danger={!validTicket}
                                class:field-success={validTicket}
                            />
                            <button
                                class="btn btn-outline"
                                disabled={!validTicket}
                                on:click={sendTicketAndUpdate}>Send</button
                            >
                            {#if tickets.length}
                                <button
                                    class="btn btn-outline"
                                    on:click={removeHandle}
                                    >Delete last ticket</button
                                >
                            {/if}
                        </div>

                        {#if tickets.length}
                            <legend class="fieldset-legend pt-2">
                                Last 5 tickets for {upStation} - {downStation}
                            </legend>
                            {#each tickets as ticket}
                                <p class="text-lg">
                                    {ticket}
                                </p>
                            {/each}
                        {/if}
                    </div>
                {/if}
            </div>
        </div>
    </div>
</main>

<style>
    .field-danger {
        border-color: red;
    }

    .field-success {
        border-color: green;
    }
</style>
