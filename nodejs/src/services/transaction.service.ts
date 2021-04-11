import DynamoDB from "aws-sdk/clients/dynamodb";
import { ConditionExpression } from "@aws/dynamodb-expressions";
import { DataMapper } from "@aws/dynamodb-data-mapper";
import { Transaction } from "../models/transaction";

export async function getBalanceTransactions(
  client: DynamoDB,
  params: { address: String; spent: Boolean }
): Promise<Transaction[]> {
  const mapper = new DataMapper({ client });
  const iterator = mapper.scan(Transaction, {
    filter: filterWithAddressAndSpent(params.address, params.spent),
  });

  let transactions = [];
  for await (const record of iterator) {
    transactions.push(record);
  }

  return transactions;
}

export function calculateTotalBalance(transactions: Transaction[]): Number {
  let balance: Number = 0;
  transactions.forEach((transaction: Transaction) => {
    if (transaction.amount) {
      balance = balance.valueOf() + transaction.amount.valueOf();
    }
  });
  return balance;
}

function filterWithAddressAndSpent(
  address: String,
  spent: Boolean
): ConditionExpression {
  const filterWithAddressAndSpent: ConditionExpression = {
    type: "And",
    conditions: [
      {
        type: "Equals",
        subject: "address",
        object: address,
      },
      {
        type: "Equals",
        subject: "spent",
        object: spent,
      },
    ],
  };

  return filterWithAddressAndSpent;
}
