import i18n from "i18next";
import { initReactI18next } from "react-i18next";

import en from "../../public/locales/en/translation.json";
import zh from "../../public/locales/zh/translation.json";

export const I18N_KEY = "i18n-key";

const resources = {
  en: {
    translation: en,
  },
  zh: {
    translation: zh,
  },
};

export const initI18n = (language: string | null) =>
  i18n.use(initReactI18next).init({
    compatibilityJSON: "v3",
    fallbackLng: "en",
    interpolation: {
      escapeValue: false,
    },
    ...(language && { lng: language }),
    resources,
  });

export default i18n;
