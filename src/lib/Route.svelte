<script lang="ts">
    import { calculatePriceAndDistance } from "./routes";
    import { getAllTickets } from "./rust_commands";
    import { cities, cityLinks } from "./routes";
    export let upStation: string = "";
    export let downStation: string = "";
    export let price: number = 0;
    export let distance: number = 0;
    export let tickets: string[] = [];

    let downStationHandle = async (upStation: string, downStation: string) => {
        [price, distance] = calculatePriceAndDistance(upStation, downStation);
        console.log(price, distance);
        tickets = await getAllTickets(upStation, downStation);
    };
</script>

<div>
    <div>
        <legend class="fieldset-legend">Boarding station</legend>
        <select
            class="select"
            bind:value={upStation}
            on:change={() => (downStation = "")}
        >
            {#each cities as city}
                <option value={city}>
                    {city}
                </option>
            {/each}
        </select>
    </div>
    <div>
        <legend class="fieldset-legend">Destination</legend>
        <select
            class="select"
            bind:value={downStation}
            on:change={() => downStationHandle(upStation, downStation)}
        >
            {#if upStation}
                {#each cityLinks[upStation] as city}
                    <option value={city}>
                        {city}
                    </option>
                {/each}
            {/if}
        </select>
    </div>
    {#if downStation}
        <legend class="font-bold">Distance: {distance} km</legend>
        <legend class="font-bold">Price: {price} RON</legend>
    {/if}
</div>
