/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      fontFamily: {
        'primary': ['Space Grotesk', '-apple-system', 'BlinkMacSystemFont', 'sans-serif'],
        'mono': ['JetBrains Mono', 'SF Mono', 'monospace'],
      },
      colors: {
        // Deep space backgrounds
        'bg-void': '#000000',
        'bg-space': '#0d1117',
        'bg-space-dark': '#0a0d13',
        'bg-surface': '#161b22',
        'bg-elevated': '#21262d',
        'bg-accent': '#30363d',
        
        // Monochrome text hierarchy
        'text-primary': '#f0f6fc',
        'text-secondary': '#c9d1d9',
        'text-tertiary': '#8b949e',
        'text-quaternary': '#6e7681',
        'text-ghost': '#484f58',
        
        // Structural elements
        'border-primary': '#30363d',
        'border-secondary': '#21262d',
        'border-accent': '#f0f6fc',
        
        // Unique accent system
        'accent-primary': '#ffffff',
        'accent-inverse': '#000000',
        
        // Status colors
        'status-success': '#238636',
        'status-warning': '#d29922',
        'status-danger': '#da3633',
      },
      spacing: {
        'xs': '0.125rem',
        'sm': '0.375rem',
        'md': '0.625rem',
        'lg': '1rem',
        'xl': '1.75rem',
        '2xl': '2.5rem',
        '3xl': '4rem',
        '4xl': '6rem',
      },
    },
  },
  plugins: [],
}
