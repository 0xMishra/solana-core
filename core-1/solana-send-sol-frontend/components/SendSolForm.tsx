import { FC, useEffect, useMemo } from "react";
import styles from "../styles/Home.module.css";
import { useConnection, useWallet } from "@solana/wallet-adapter-react";
import * as Web3 from "@solana/web3.js";
import { useState } from "react";
export const SendSolForm: FC = () => {
  const [recipient, setRecipient] = useState("");
  const [amount, setAmount] = useState("");
  const [sendersBalance, setSendersBalance] = useState("");
  const { connection } = useConnection();
  const { publicKey, sendTransaction } = useWallet();

  useEffect(() => {
    if (!connection || !publicKey) {
      return;
    }

    connection.getBalance(publicKey).then((info) => {
      setSendersBalance(info.toString());
    });
  }, [connection, publicKey]);

  const sendSol = async (event) => {
    event.preventDefault();
    if (!connection || !publicKey) {
      console.log("please connect the wallet ");
      return;
    }
    const transaction = new Web3.Transaction();
    const instruction = Web3.SystemProgram.transfer({
      fromPubkey: publicKey,
      toPubkey: new Web3.PublicKey(recipient),
      lamports: parseFloat(amount) * Web3.LAMPORTS_PER_SOL,
    });

    transaction.add(instruction);
    const sig = await sendTransaction(transaction, connection);
    console.log(
      `You can view your transaction on the Solana Explorer at:\nhttps://explorer.solana.com/tx/${sig}?cluster=devnet`
    );

    connection.getBalance(publicKey).then((info) => {
      setSendersBalance(info.toString());
    });
  };

  return (
    <div>
      <form onSubmit={sendSol} className={styles.form}>
        <label htmlFor="amount">
          Amount (in SOL) to send:{" "}
          {parseFloat(sendersBalance) / Web3.LAMPORTS_PER_SOL}
        </label>
        <input
          id="amount"
          type="text"
          className={styles.formField}
          placeholder="e.g. 0.1"
          onChange={(e) => setAmount(e.target.value)}
          required
        />
        <br />
        <label htmlFor="recipient">Send SOL to:</label>
        <input
          id="recipient"
          type="text"
          className={styles.formField}
          placeholder="e.g. 4Zw1fXuYuJhWhu9KLEYMhiPEiqcpKd6akw3WRZCv84HA"
          onChange={(e) => setRecipient(e.target.value)}
          required
        />
        <button type="submit" className={styles.formButton} onClick={sendSol}>
          Send
        </button>
      </form>
    </div>
  );
};
