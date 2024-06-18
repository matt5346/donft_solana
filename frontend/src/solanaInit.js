import { Connection, clusterApiUrl } from "@solana/web3.js";

const connection = new Connection(clusterApiUrl(process.env.VUE_APP_WEB3), "confirmed");

export default connection;
