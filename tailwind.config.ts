import type { Config } from 'tailwindcss';

export default {
  content: ['./index.html', './src/**/*.{js,ts,jsx,tsx}'],
  theme: {
    extend: {
      fontFamily: {
        sans: ['Inter', 'ui-sans-serif', 'system-ui', '-apple-system', 'BlinkMacSystemFont', 'Segoe UI', 'sans-serif'],
      },
      boxShadow: {
        glass: '0 24px 80px rgba(0,0,0,0.35)',
      },
      colors: {
        ink: '#07080d',
      },
    },
  },
  plugins: [],
} satisfies Config;
