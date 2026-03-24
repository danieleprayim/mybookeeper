/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}", // your React components
  ],
  theme: {
    extend: {
      colors: {
        primary: "#2563EB",       // blue for buttons / highlights
        secondary: "#1E293B",     // dark sidebar background
        accent: "#FBBF24",        // accent color (e.g., for alerts)
        bg: "#F3F4F6",            // main background
      },
      fontFamily: {
        sans: ["Inter", "system-ui", "sans-serif"],
      },
      spacing: {
        sidebar: "16rem",          // 64px width for sidebar
        topbar: "4rem",            // 16px height for topbar
      },
      borderRadius: {
        card: "0.5rem",            // rounded corners for cards
      },
      boxShadow: {
        card: "0 1px 4px rgba(0,0,0,0.1)",
      },
    },
  },
  plugins: [],
  corePlugins: {
    preflight: true,  // include base styles for reset
  },
};