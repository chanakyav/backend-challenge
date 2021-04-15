import express from "express";
import DynamoDB from "aws-sdk/clients/dynamodb";
import { getBalance } from "./services/transaction.service";

let client: DynamoDB;

console.log("Connecting to DynamoDB client...");
client = new DynamoDB({ region: "us-east-1" });
console.log("Connected!");

const app = express();
app.use(express.json());

app.get("/balance", async (req, res) => {
  if (req.body.address === undefined && req.body.spent === undefined) {
    return res.status(400).send({
      error:
        "/balance requires the following keys within JSON body - address: String, spent: Boolean",
    });
  }

  const balance = await getBalance(client, req.body);
  return res.send({
    balance: balance,
  });
});

app.listen(3000, () => {
  console.log("The application is listening on port 3000!");
});
