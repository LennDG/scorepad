const { fontFamily } = require("tailwindcss/defaultTheme");

/** @type {import('tailwindcss').Config} */
module.exports = {
    content: {
        files: ["*.html", "../src/**/*.rs"],
    },
    theme: {
        extend: {
            fontFamily: {
                sans: ["Inter var", ...fontFamily.sans],
            },
        },
    },
    plugins: [],
}