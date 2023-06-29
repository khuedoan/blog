/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.rs",
  ],
  theme: {},
  plugins: [
    require("@tailwindcss/typography"),
  ],
};
