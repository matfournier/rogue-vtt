// eslint-disable-next-line import/no-unresolved
// @ts-ignore
import { registerSW } from "virtual:pwa-register";

const updateSw = registerSW({
  onNeedRefresh() {
    const isUpdateApproved = window.confirm(
      "New app version available. Reload?",
    );
    if (isUpdateApproved) {
      updateSw(true);
    }
  },
});