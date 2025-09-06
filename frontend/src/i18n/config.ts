import i18n from "i18next";
import { initReactI18next } from "react-i18next";
import LanguageDetector from "i18next-browser-languagedetector";

import esTranslation from "../locales/es/translation.json";
import enTranslation from "../locales/en/translation.json";

i18n
  .use(LanguageDetector)
  .use(initReactI18next)
  .init({
    debug: true,
    fallbackLng: "en",
    interpolation: {
      escapeValue: false,
    },
    resources: {
      es: {
        translation: esTranslation,
      },
      en: {
        translation: enTranslation,
      },
    },
  });

export default i18n;
