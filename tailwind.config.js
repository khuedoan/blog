/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.rs",
  ],
  theme: {
    extend: {
      typography: {
        DEFAULT: {
          css: {
            // TODO make the in-line code look better
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
