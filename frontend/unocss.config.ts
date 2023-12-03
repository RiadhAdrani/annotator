import presetUno from '@unocss/preset-uno';
import { defineConfig, presetWind, presetIcons } from 'unocss';

export default defineConfig({
  presets: [
    presetUno(),
    presetWind(),
    presetIcons({
      extraProperties: {
        display: 'inline-block',
        'vertical-align': 'middle',
      },
    }),
  ],
  shortcuts: [
    {
      col: 'flex flex-col',
      row: 'flex flex-row',

      'col-center': 'col justify-center items-center',
      'row-center': 'row justify-center items-center',
    },
  ],
});
