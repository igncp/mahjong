import dayjs from "dayjs";
import localeEN from "dayjs/locale/en";
import localeZH from "dayjs/locale/zh-tw";
import relativeTime from "dayjs/plugin/relativeTime";

import { InternalLocale } from "./i18n";

const internalLocaleToDayjsLocale: Record<InternalLocale, ILocale> = {
  en: localeEN,
  zh: localeZH,
};

dayjs.extend(relativeTime);

export const simpleFormatDate = (timestamp: string): string => {
  const timestampNum = Number(timestamp);

  if (isNaN(timestampNum)) {
    return "-";
  }

  const day = dayjs(timestampNum);

  return day.format("YYYY-MM-DD HH:mm:ss");
};

export const simpleFormatDateSince = (timestamp: string): string => {
  const timestampNum = Number(timestamp);

  if (isNaN(timestampNum)) {
    return "";
  }

  const day = dayjs(timestampNum);

  return day.fromNow();
};

export const setTimeLocale = (locale: InternalLocale): void => {
  if (!internalLocaleToDayjsLocale[locale]) {
    console.error(`Locale ${locale} is not supported.`);
    return;
  }

  dayjs.locale(internalLocaleToDayjsLocale[locale]);
};
