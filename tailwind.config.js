/** @type {import('tailwindcss').Config} */
module.exports = {
    mode: "all",
    content: [
        // include all rust, html and css files in the src directory
        "./src/**/*.{rs,html,css}",
        // include all html files in the output (dist) directory
        "./dist/**/*.html",
    ],
    theme: {
        fontSize: {
            sm: '7vmin',
            base: '10vmin',
            lg: '25vmin',
            xl: '40vmin',
        },
        extend: {
            fontFamily: {
                dice: ["dice", "monospace"],
            }
        },
    },
    plugins: [],
}
