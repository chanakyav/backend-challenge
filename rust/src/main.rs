use rusoto_core::Region;
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, ScanInput};
use std::collections::HashMap;

const TABLE_NAME: &str = "transactions";
const FILTER_WITH_ADDRESS_AND_SPENT: &str = "address = :address_value and spent = :spent_value";

#[tokio::main]
async fn main() {
    let client = DynamoDbClient::new(Region::UsEast1);

    let address = String::from("1CL5TbB2MaR4mrFjtYQ5GyA3cP2bSmPxAn".to_string());
    let spent = true;
    let scan_input = create_scan_input(address, spent);

    let mut items: Vec<HashMap<String, AttributeValue>> = Vec::new();
    match client.scan(scan_input).await {
        Ok(output) => {
            match output.items {
                Some(item_list) => {
                    for item in item_list {
                        items.push(item);
                    }
                }
                None => println!("No transactions found with the given address and spent values"),
            }
            match output.last_evaluated_key {
                Some(key) => {
                    println!("Last evaluated key: {:?}", key)
                }
                None => println!("No more data to be retrieved"),
            }
        }
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }

    println!("Balance: {}", calculate_total_balance(items));
}

fn create_scan_input(address: String, spent: bool) -> ScanInput {
    let mut address_value: AttributeValue = Default::default();
    address_value.s = Some(address);
    let mut spent_value: AttributeValue = Default::default();
    spent_value.bool = Some(spent);

    let mut values: HashMap<String, AttributeValue> = HashMap::new();
    values.insert(":address_value".to_string(), address_value);
    values.insert(":spent_value".to_string(), spent_value);

    let mut scan_input: ScanInput = Default::default();
    scan_input.table_name = String::from(TABLE_NAME);
    scan_input.filter_expression = Some(String::from(FILTER_WITH_ADDRESS_AND_SPENT));
    scan_input.expression_attribute_values = Some(values);
    scan_input.projection_expression = Some("amount".to_string());

    scan_input
}

fn calculate_total_balance(items: Vec<HashMap<String, AttributeValue>>) -> f64 {
    let mut amount: f64 = 0.0;
    for item in items {
        match item.get(&"amount".to_string()) {
            Some(amt) => match &amt.n {
                Some(val) => {
                    amount = amount + val.parse::<f64>().unwrap();
                }
                None => println!("No amount value"),
            },
            None => println!("No attribute value"),
        }
    }
    amount
}
