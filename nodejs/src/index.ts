import express from "express";
import DynamoDB from "aws-sdk/clients/dynamodb";
import {
  getBalanceTransactions,
  calculateTotalBalance,
} from "./services/transaction.service";

let client: DynamoDB;
(() => {
  console.log("Connecting to DynamoDB client...");
  client = new DynamoDB({ region: "us-east-1" });
  console.log("Connected!");
})();

const app = express();
app.use(express.json());

app.get("/balance", async (req, res) => {
  if (req.body.address === undefined && req.body.spent === undefined) {
    return res.status(400).send({
      error:
        "/balance requires the following keys within JSON body - address: String, spent: Boolean",
    });
  }

  const transactions = await getBalanceTransactions(client, req.body);
  if (transactions.length === 0) {
    return res.status(204).send({
      info: "No transactions were found with given address or spent type",
    });
  }

  const balance = calculateTotalBalance(transactions);
  return res.send(balance.toString());
});

app.listen(3000, () => {
  console.log("The application is listening on port 3000!");
});
