import type { LanguageOption } from "@shared/types";

export const DEFAULT_LANGUAGES: LanguageOption[] = [
  { code: "auto", name: "Auto detect", native_name: "Auto" },
  { code: "en", name: "English", native_name: "English" },
  { code: "ru", name: "Russian", native_name: "Русский" },
  { code: "es", name: "Spanish", native_name: "Español" },
  { code: "de", name: "German", native_name: "Deutsch" },
  { code: "fr", name: "French", native_name: "Français" },
  { code: "it", name: "Italian", native_name: "Italiano" },
  { code: "pt", name: "Portuguese", native_name: "Português" },
  { code: "nl", name: "Dutch", native_name: "Nederlands" },
  { code: "pl", name: "Polish", native_name: "Polski" },
  { code: "uk", name: "Ukrainian", native_name: "Українська" },
  { code: "tr", name: "Turkish", native_name: "Türkçe" },
  { code: "ja", name: "Japanese", native_name: "日本語" },
  { code: "zh", name: "Chinese (Simplified)", native_name: "中文" },
  { code: "ko", name: "Korean", native_name: "한국어" },
  { code: "ar", name: "Arabic", native_name: "العربية" },
  { code: "hi", name: "Hindi", native_name: "हिन्दी" },
  { code: "id", name: "Indonesian", native_name: "Bahasa Indonesia" },
  { code: "vi", name: "Vietnamese", native_name: "Tiếng Việt" },
  { code: "th", name: "Thai", native_name: "ไทย" },
  { code: "he", name: "Hebrew", native_name: "עברית" },
  { code: "cs", name: "Czech", native_name: "Čeština" },
  { code: "sv", name: "Swedish", native_name: "Svenska" },
  { code: "fi", name: "Finnish", native_name: "Suomi" },
  { code: "da", name: "Danish", native_name: "Dansk" },
  { code: "no", name: "Norwegian", native_name: "Norsk" },
  { code: "el", name: "Greek", native_name: "Ελληνικά" },
  { code: "ro", name: "Romanian", native_name: "Română" },
  { code: "hu", name: "Hungarian", native_name: "Magyar" },
];

export function findLanguage(code: string, list: LanguageOption[] = DEFAULT_LANGUAGES): LanguageOption | undefined {
  return list.find((l) => l.code === code);
}
