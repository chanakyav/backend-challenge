use rusoto_core::Region;
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, ScanInput};
use std::collections::HashMap;

const TABLE_NAME: &str = "transactions";
const FILTER_WITH_ADDRESS_AND_SPENT: &str = "address = :address_value and spent = :spent_value";

#[tokio::main]
async fn main() {
    let client = DynamoDbClient::new(Region::UsEast1);

    println!("scan items for address and spent");
    let mut address_value: AttributeValue = Default::default();
    address_value.s = Some("335bRXoRfGcXAdoHi57MpRmWDPgChAPncn".to_string());
    let mut spent_value: AttributeValue = Default::default();
    spent_value.bool = Some(true);

    let mut values: HashMap<String, AttributeValue> = HashMap::new();
    values.insert(":address_value".to_string(), address_value);
    values.insert(":spent_value".to_string(), spent_value);
    let mut scan_input: ScanInput = Default::default();
    scan_input.table_name = String::from(TABLE_NAME);
    scan_input.filter_expression = Some(String::from(FILTER_WITH_ADDRESS_AND_SPENT));
    scan_input.expression_attribute_values = Some(values);

    match client.scan(scan_input).await {
        Ok(output) => match output.items {
            Some(item_list) => {
                println!("Item count: {}", item_list.len());
                for item in item_list {
                    println!("Item: {:?}", item);
                }
            }
            None => println!("No transactions found with the given address and spent values"),
        },
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
}
