/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.rs",
  ],
  theme: {
    // TODO make code looks better
  },
  plugins: [
    require("@tailwindcss/typography"),
  ],
};
