/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{js,jsx}"],
  theme: {
    extend: {
      colors: {
        porcelain: "#F7F4EE",
        ink: "#26261F",
        teal: {
          DEFAULT: "#1F4E5F",
          dark: "#173B48",
          light: "#3D7286",
        },
        brick: "#C8553D",
        sage: "#4C7A5B",
        amber: "#C08A2E",
        line: "#E4DFD3",
      },
      fontFamily: {
        display: ["Fraunces", "serif"],
        sans: ["Inter", "system-ui", "sans-serif"],
        mono: ["IBM Plex Mono", "monospace"],
      },
    },
  },
  plugins: [],
};
