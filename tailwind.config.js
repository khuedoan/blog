/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./app/**/*.{js,ts,jsx,tsx}",
    "./components/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      typography: {
        DEFAULT: {
          css: {
            code: {
              backgroundColor: "#eff1f3",
              "border-radius": "0.25rem",
              fontWeight: 400,
            },
            "code::before": {
              content: '""',
            },
            "code::after": {
              content: '""',
            },
            "pre": null,
            "pre code": null,
            "pre code::before": null,
            "pre code::after": null,
          },
        },
      },
    },
  },
  plugins: [
    require("@tailwindcss/typography"),
  ],
};
