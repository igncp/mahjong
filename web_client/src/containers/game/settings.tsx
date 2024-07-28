import type { GameSettings } from "bindings/GameSettings";
import type { ServiceGameSummary } from "bindings/ServiceGameSummary";
import { useMemo } from "react";
import { useTranslation } from "react-i18next";

import type { ModelServiceGameSummary } from "src/sdk/service-game-summary";
import type { SelectOption } from "src/ui/common/select";
import Select from "src/ui/common/select";
import Text from "src/ui/common/text";

import styles from "./settings.module.scss";

const convertDiscardWaitMsValue = (value: GameSettings["discard_wait_ms"]) => {
  if (value === null) return "none";

  if (value === 1000) return "1s";

  if (value === 10000) return "10s";

  if (value === 60000) return "1m";

  if (value === -1) return "block";

  return "none";
};

type IProps = {
  serviceGameM: ModelServiceGameSummary;
  serviceGameSummary: ServiceGameSummary;
};

const Settings = ({ serviceGameM, serviceGameSummary }: IProps) => {
  const { i18n, t } = useTranslation();

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const onAIEnabledChanged = (event: any) => {
    const aiEnabled = event.target.value === "enabled";

    serviceGameM.setGameSettings({
      ...serviceGameSummary.settings,
      ai_enabled: aiEnabled,
    });
  };

  const onDiscardWaitMsChanged = (value: string) => {
    const msValue = (() => {
      switch (value) {
        case "1s":
          return 1000;
        case "10s":
          return 10000;
        case "1m":
          return 60000;
        case "block":
          return -1;
        default:
          return null;
      }
    })();

    serviceGameM.setGameSettings({
      ...serviceGameSummary.settings,
      discard_wait_ms: msValue,
    });
  };

  const autoSortOptions: SelectOption[] = useMemo(
    () => [
      {
        label: t("game.option.yes"),
        value: "yes",
      },
      {
        label: t("game.option.no"),
        value: "no",
      },
    ],
    [t],
  );

  const autoStopDrawMeldOptions: SelectOption[] = useMemo(
    () => [
      {
        label: t("game.option.yes"),
        value: "yes",
      },
      {
        label: t("game.option.no"),
        value: "no",
      },
    ],
    [t],
  );

  const discardWaitMsOptions: SelectOption[] = useMemo(
    () => [
      {
        label: t("game.wait.none"),
        value: "none",
      },
      {
        label: t("game.wait.1sec"),
        value: "1s",
      },
      {
        label: t("game.wait.10sec"),
        value: "10s",
      },
      {
        label: t("game.wait.1min"),
        value: "1m",
      },
      {
        label: t("game.wait.block"),
        value: "block",
      },
    ],
    [t],
  );

  const onAutoSortChange = (value: string) => {
    const boolValue = value === "yes";

    serviceGameM.setGameSettings({
      ...serviceGameSummary.settings,
      auto_sort: boolValue,
    });
  };

  const onAutoStopDrawMeldChange = (value: string) => {
    const boolValue = value === "yes";

    serviceGameM.setGameSettings({
      ...serviceGameSummary.settings,
      auto_stop_claim_meld: boolValue,
    });
  };

  return (
    <>
      {/* Adding the language as the key or there are some issues with
      react-select when selecting an option and then changing language */}
      <form className={styles.cardContentSettings} key={i18n.language}>
        <div className={styles.settingsFormInner}>
          <Text>
            <b>{t("game.AI.title")}</b>:{" "}
            <label style={{ marginRight: "10px" }}>
              {t("game.AI.enabled")}
              <input
                checked={serviceGameSummary.settings.ai_enabled}
                name="ai_enabled"
                onChange={onAIEnabledChanged}
                type="radio"
                value="enabled"
              />
            </label>
            <label>
              {t("game.AI.disabled")}
              <input
                checked={!serviceGameSummary.settings.ai_enabled}
                name="ai_enabled"
                onChange={onAIEnabledChanged}
                type="radio"
                value="disabled"
              />
            </label>
          </Text>
          <Text>{t("game.blockTime.desc")}: </Text>
          <Select
            defaultValue={
              convertDiscardWaitMsValue(
                serviceGameSummary.settings.discard_wait_ms,
              ) || "none"
            }
            disabled={!serviceGameSummary.settings.ai_enabled}
            onChange={onDiscardWaitMsChanged}
            options={discardWaitMsOptions}
            style={{ width: 120 }}
          />
          <Text>{t("game.autoSort")}</Text>
          <Select
            defaultValue={serviceGameSummary.settings.auto_sort ? "yes" : "no"}
            disabled={false}
            onChange={onAutoSortChange}
            options={autoSortOptions}
            style={{ width: 120 }}
          />
          <Text>{t("game.autoStopDrawMeld")}</Text>
          <Select
            defaultValue={
              serviceGameSummary.settings.auto_stop_claim_meld ? "yes" : "no"
            }
            disabled={false}
            onChange={onAutoStopDrawMeldChange}
            options={autoStopDrawMeldOptions}
            style={{ width: 120 }}
          />
        </div>
      </form>
    </>
  );
};

export default Settings;
