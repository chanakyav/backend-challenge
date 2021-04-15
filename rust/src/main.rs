use rusoto_core::Region;
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, ScanInput};
use std::collections::HashMap;
use std::io;

const TABLE_NAME: &str = "transactions";
const FILTER_WITH_ADDRESS_AND_SPENT: &str = "address = :address_value and spent = :spent_value";

#[tokio::main]
async fn main() {
    let client = DynamoDbClient::new(Region::UsEast1);

    println!("Enter address value:");
    let mut address_input = String::new();
    io::stdin()
        .read_line(&mut address_input)
        .expect("Failed to read address. Enter a string, ex. 1CL5TbB2MaR4mrFjtYQ5GyA3cP2bSmPxAn");

    println!("Enter spent value:");
    let mut spent_input = String::new();
    io::stdin()
        .read_line(&mut spent_input)
        .expect("Failed to read input");

    let address: &str = address_input.trim();
    let spent: bool = spent_input.trim().parse().expect("Enter boolean values, ex. false");
    
    /* 
        Created a mutable scan_input variable because the scan function goes through DynamoDB
        using pagination so we need to update the next key from which we run the scan from.
        When we first run scan, there is no last evaluated key, so we start with None
    */
    let mut scan_input: ScanInput = Default::default();
    update_scan_input(&mut scan_input, address, spent, None);


    let mut items: Vec<HashMap<String, AttributeValue>> = Vec::new();
    // We loop until there is no last evaluated key
    'outer: loop {
        match client.scan(scan_input.clone()).await {
            Ok(output) => {
                match output.items {
                    Some(item_list) => {
                        for item in item_list {
                            items.push(item);
                        }
                    }
                    None => {
                        println!("No transactions found with the given address and spent values")
                    }
                }
                match output.last_evaluated_key {
                    Some(obj) => match obj.get("id") {
                        Some(attribute_value) => match &attribute_value.s {
                            Some(value) => {
                                update_scan_input(
                                    &mut scan_input,
                                    address,
                                    spent,
                                    Some(value.to_string()),
                                );
                            }
                            None => println!("No id found"),
                        },
                        None => println!("No attribute value found"),
                    },
                    None => {
                        break 'outer;
                    }
                }
            }
            Err(error) => {
                println!("Error: {:?}", error);
            }
        }
    }

    println!("Balance: {}", calculate_total_balance(items));
}

fn update_scan_input(
    scan_input: &mut ScanInput,
    address: &str,
    spent: bool,
    last_evaluated_key: Option<String>,
) {
    match last_evaluated_key {
        Some(key) => {
            let mut start_key_value: AttributeValue = Default::default();
            start_key_value.s = Some(key);
            let mut start_key: HashMap<String, AttributeValue> = HashMap::new();
            start_key.insert("id".to_string(), start_key_value);
            scan_input.exclusive_start_key = Some(start_key);
        }
        None => {
            let mut address_value: AttributeValue = Default::default();
            address_value.s = Some(address.to_string());
            let mut spent_value: AttributeValue = Default::default();
            spent_value.bool = Some(spent);

            let mut values: HashMap<String, AttributeValue> = HashMap::new();
            values.insert(":address_value".to_string(), address_value);
            values.insert(":spent_value".to_string(), spent_value);

            scan_input.table_name = String::from(TABLE_NAME);
            scan_input.filter_expression = Some(String::from(FILTER_WITH_ADDRESS_AND_SPENT));
            scan_input.expression_attribute_values = Some(values);
            scan_input.projection_expression = Some("amount".to_string());
        }
    }
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
