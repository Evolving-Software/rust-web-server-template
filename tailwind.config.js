/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: "media", // or 'media' or 'class'
  content: ["./templates/**/*.{html.tera,js}"],
  theme: {
    extend: {

    },
  },
  plugins: [
    require('@tailwindcss/forms'),
  ],
}
