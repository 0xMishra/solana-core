import type { AppProps } from "next/app";
import { ChakraProvider } from "@chakra-ui/react";
import { extendTheme } from "@chakra-ui/react";
import WalletContextProvider from "@/components/WalletContextProvider";

const colors = {
  background: "#1f1f1f",
  accent: "833BBE",
  bodyText: "rgba(255,255,255,0.75)",
};

const theme = extendTheme({ colors });

export default function App({ Component, pageProps }: AppProps) {
  return (
    <ChakraProvider theme={extendTheme}>
      <WalletContextProvider>
        <Component {...pageProps} />;
      </WalletContextProvider>
    </ChakraProvider>
  );
}
