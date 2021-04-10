import { DynamoDbSchema, DynamoDbTable } from "@aws/dynamodb-data-mapper";

const TABLE_NAME = "transactions";

export class Transaction {
  id: String | undefined;
  txid: String | undefined;
  address: String | undefined;
  amount: Number | undefined;
  spent: Boolean | undefined;
}

Object.defineProperties(Transaction.prototype, {
  [DynamoDbTable]: {
    value: TABLE_NAME,
  },
  [DynamoDbSchema]: {
    value: {
      id: {
        type: "String",
        keyType: "HASH",
      },
      txid: { type: "String" },
      address: { type: "String" },
      amount: { type: "Number" },
      spent: { type: "Boolean" },
    },
  },
});
