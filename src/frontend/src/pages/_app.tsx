import "@/styles/globals.css";
import type { AppProps } from "next/app";

import { ThemeProvider } from 'next-themes';

export default function MyApp({ Component, pageProps }: any) {
  return (
    <ThemeProvider
      attribute="class"
      defaultTheme="dark" // Set default theme to dark
      enableSystem={false} // Optional: Disable system preference
    >
      <Component {...pageProps} />
    </ThemeProvider>
  );
}