import { init, addMessages } from 'svelte-i18n';
import en from '../locales/en.json';
import zh_TW from '../locales/zh-TW.json';
import oan_TW from '../locales/oan-TW.json';

addMessages('en', en);
addMessages('zh-TW', zh_TW);
addMessages('oan-TW', oan_TW);

init({
  fallbackLocale: 'en',
  initialLocale: 'zh-TW',
});