import { FC, useState } from "react";
import styles from "../styles/Home.module.css";
import { useConnection, useWallet } from "@solana/wallet-adapter-react";
import * as Web3 from "@solana/web3.js";
export const SendSolForm: FC = () => {
  const { connection } = useConnection();
  const { publicKey, sendTransaction } = useWallet();
  const [amountToSend, setAmountToSend] = useState<number | undefined>(0);
  const [recipient, setRecipient] = useState("");

  const sendSol = (event) => {
    if (!connection || !publicKey) {
      alert("Connect your wallet first");
      return;
    }
    const transaction = new Web3.Transaction();
    const instruction = Web3.SystemProgram.transfer({
      fromPubkey: publicKey,
      toPubkey: new Web3.PublicKey(recipient),
      lamports: amountToSend * Web3.LAMPORTS_PER_SOL,
    });
    transaction.add(instruction);
    event.preventDefault();
    console.log(
      `Sending ${event.target.amount.value} SOL to ${event.target.recipient.value}`
    );
    sendTransaction(transaction, connection).then((sig) => {
      console.log(
        `Transaction: https://explorer.solana.com/tx/${sig}?cluster=devnet`
      );
    });
  };

  return (
    <div>
      <form onSubmit={sendSol} className={styles.form}>
        <label htmlFor="amount">Amount (in SOL) to send:</label>
        <input
          id="amount"
          type="text"
          className={styles.formField}
          placeholder="e.g. 0.1"
          required
          onChange={(event) => setAmountToSend(parseFloat(event.target.value))}
        />
        <br />
        <label htmlFor="recipient">Send SOL to:</label>
        <input
          id="recipient"
          type="text"
          className={styles.formField}
          placeholder="e.g. 4Zw1fXuYuJhWhu9KLEYMhiPEiqcpKd6akw3WRZCv84HA"
          required
          onChange={(event) => setRecipient(event.target.value)}
        />
        <button type="submit" className={styles.formButton}>
          Send
        </button>
      </form>
    </div>
  );
};
