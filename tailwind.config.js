/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
  // applies tailwind css styles from rust files in these conditions
    "./src/**/*.rs", 
    "./src/app/**/*.rs", 
    "./src/app/*.rs"
  ],
  theme: {
    extend: {},
  },
  plugins: [
    require('@tailwindcss/typography'),
    require('@tailwindcss/forms'),
    require('@tailwindcss/aspect-ratio'),
    require('@tailwindcss/container-queries'),
  ],
}
