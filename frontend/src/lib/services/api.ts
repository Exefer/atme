import type { ChargeRecord } from "$lib/types/backend";
import ky from "ky";

const client = ky.create({
  prefixUrl: "/api",
  timeout: 30000
});

export const api = {
  phone: {
    getChargeRecords() {
      return client.get("phone/charge-records").json<ChargeRecord[]>();
    }
  }
};
