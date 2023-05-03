import { init, addMessages } from "svelte-i18n";
import en from "../locales/en.json";
import oan_Han from "../locales/oan_Han.json";
import oan_Latn from "../locales/oan_Latn.json";

addMessages("en", en);
addMessages("oan_Han", oan_Han);
addMessages("oan_Latn", oan_Latn);

init({
    fallbackLocale: "en",
    initialLocale: "oan_Han",
});
