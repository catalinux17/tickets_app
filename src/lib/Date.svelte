<script lang="ts">
    import { DateInput } from "date-picker-svelte";
    export let date: Date = new Date();
</script>

<div>
    <legend class="fieldset-legend">Date</legend>
    <DateInput closeOnSelection={true} format="yyyy-MM-dd" bind:value={date} />
</div>
