import { useState, useEffect } from "react";
import { NextPage } from "next";
import styles from "../styles/Home.module.css";
import { AppBar } from "../components/AppBar";
import { SendSolForm } from "../components/SendSolForm";
import Head from "next/head";
import { useConnection, useWallet } from "@solana/wallet-adapter-react";
import * as Web3 from "@solana/web3.js";
const Home: NextPage = (props) => {
  const { connection } = useConnection();
  const { publicKey } = useWallet();
  const [sendersBalance, setSendersBalance] = useState<number | undefined>(0);

  const getSendersBalance = async () => {
    const currentBalance = await connection.getBalance(publicKey);
    setSendersBalance(currentBalance / Web3.LAMPORTS_PER_SOL);
  };
  useEffect(() => {
    getSendersBalance();
  }, []);
  return (
    <div className={styles.App}>
      <Head>
        <title>Wallet-Adapter Example</title>
        <meta name="description" content="Wallet-Adapter Example" />
      </Head>
      <AppBar />
      <div className={styles.AppBody}>
        <p>Your Balance: {sendersBalance} SOL</p>
        <SendSolForm />
      </div>
    </div>
  );
};

export default Home;
