"use strict";
var _a;
Object.defineProperty(exports, "__esModule", { value: true });
exports.Transaction = void 0;
var dynamodb_data_mapper_1 = require("@aws/dynamodb-data-mapper");
var TABLE_NAME = "transactions";
var Transaction = /** @class */ (function () {
    function Transaction() {
    }
    return Transaction;
}());
exports.Transaction = Transaction;
Object.defineProperties(Transaction.prototype, (_a = {},
    _a[dynamodb_data_mapper_1.DynamoDbTable] = {
        value: TABLE_NAME,
    },
    _a[dynamodb_data_mapper_1.DynamoDbSchema] = {
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
    _a));
