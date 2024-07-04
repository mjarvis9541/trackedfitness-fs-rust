/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    relative: true,
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {
      gridTemplateColumns: {
        "checkbox-4": "auto repeat(4, 1fr);",
        "checkbox-6": "auto repeat(6, 1fr);",
        "checkbox-8": "auto repeat(8, 1fr);",
        "checkbox-9": "auto repeat(9, 1fr);",
        "checkbox-10": "auto repeat(10, 1fr);",
        "checkbox-12": "auto repeat(12, 1fr);",
        "checkbox-14": "auto repeat(14, 1fr);",
        "checkbox-16": "auto repeat(15, 1fr);",
        "input-12": "2fr repeat(12, minmax(0, 1fr));",
      },
      gridTemplateRows: {
        "row-hide": "0fr;",
        "row-show": "1fr;",
      },
      transitionProperty: {
        "width": "width",
        "grid-template-rows": "grid-template-rows",
      },
    },
  },
  plugins: [],
};
