<script lang="ts">
  import type { ChargeRecord } from "$lib/types/backend/bindings";
  import { createQuery } from "@tanstack/svelte-query";
  import { derived } from "svelte/store";
  import {
    calculateDuration,
    calculateStats,
    formatDurationFromMs,
    formatPercentage,
    formatTimestamp
  } from "./helpers";

  const refetchInterval = 2000;

  const query = createQuery<ChargeRecord[]>({
    queryKey: ["phone_charge-records"],
    queryFn: () => getChargeRecords(),
    initialData: [],
    refetchInterval
  });

  const getChargeRecords = async () => {
    const response = await fetch("/api/phone/charge-records");
    return response.json();
  };

  const stats = derived(query, ($query) => {
    return calculateStats($query.data);
  });
</script>

<main class="flex flex-col gap-4 p-4">
  <ul>
    <li>Total Records: {$stats.totalRecords}</li>
    <li>Average Charge Duration: {formatDurationFromMs($stats.avgDuration) || "-"}</li>
    <li>Average Charge Percentage: {formatPercentage($stats.avgChargePercentage)}</li>
  </ul>
  <div class="rounded-box border-base-content/5 bg-base-100 overflow-x-auto border">
    <table class="table">
      <thead>
        <tr>
          <th scope="col">Start Time</th>
          <th scope="col">End Time</th>
          <th scope="col">Duration</th>
          <th scope="col">Start Percentage</th>
          <th scope="col">End Percentage</th></tr
        >
      </thead>
      <tbody>
        {#snippet entry(record: ChargeRecord)}
          <tr>
            <td>{formatTimestamp(record.start_timestamp)}</td>
            <td>{formatTimestamp(record.end_timestamp)}</td>
            <td>{calculateDuration(record.start_timestamp, record.end_timestamp)}</td>
            <td>{formatPercentage(record.start_percentage, 0)}</td>
            <td>{formatPercentage(record.end_percentage, 0)}</td>
          </tr>
        {/snippet}
        {#each $query.data as record}
          {@render entry(record)}
        {/each}
      </tbody>
    </table>
  </div>
</main>
