use reqwest::Client;
use std::error::Error;
use std::collections::HashMap;
use urlencoding::encode;
use serde::Serialize;


async fn create_payment_intent(amount: i64, currency: &str) -> Result<String, Box<dyn Error>> {

    let grazie_key = "rk_test_51Of65SKXKrWylrLogBawgLHtpKHGIR8odhDk5omPfy5QxA2soIKqhUKOIbHbcPbKSkrH466D4NPgG5TAxoUvW0yk00km8ftile";
    let api_key = grazie_key;

    let client = Client::new();

    println!("#1");

    // Prepare the parameters
    let params = PaymentIntentParams {
        amount: 2000,
        currency: "eur".to_string(),
        payment_method_types: vec!["card".to_string(), "bank_transfer".to_string()],
    };

/*
    let params = &[
    ("amount", "2000"),
    ("currency", "eur"),
    ("payment_method_types", "klarna"),
    ("payment_method_types", "link")
];
*/
    // Serialize the parameters to application/x-www-form-urlencoded format
    let form_data2 = serde_urlencoded::to_string(params)?;

    println!("form_data2: {}", form_data2);



    println!("#2");

    // Serialize the parameters to application/x-www-form-urlencoded format
    //let form_data2 = serde_urlencoded::to_string(&params)?;

    println!("#3");

    

    // Manually construct the URL-encoded string for basic parameters
    let amount = 2000;
    let currency = "eur";
    let payment_method_types = vec!["klarna", "link"];

    let mut form_data_str = format!("amount={}&currency={}", encode(&amount.to_string()), encode(currency));

    // Manually add payment_method_types[] to the form data string with indexes
    


    for method in payment_method_types {
        form_data_str.push('&');
        form_data_str.push_str(&format!("payment_method_types[]={}", encode(method)));
    }


/*

    // Create a HashMap to store the form data
    let mut form_data = HashMap::new();

    // Add single key-value pairs
    form_data.insert("amount", "2000");
    form_data.insert("currency", "eur");

    // Add an array by repeating the same key with square brackets
    let payment_method_types = vec!["klarna", "link"];
    for method in payment_method_types {
        form_data.insert("payment_method_types[]", method);
    }
    */

    

    println!("form_data {}", form_data_str);
    
    let response = client
        .post("https://api.stripe.com/v1/payment_intents")
        .bearer_auth(api_key)
        .header("Content-type", "application/x-www-form-urlencoded")
        .form(&params)
        .send()
        .await?;

    //println!("{:?}", response);

    let body = response.text().await?; // Read the response body as a String
    println!("Response:\n{}", body);


    Ok(body)

    //Ok(())
}

#[tokio::main]
async fn main() {
    let payment_intent = create_payment_intent(2000, "usd").await;
    match payment_intent {
        Ok(intent) => println!("Created Payment Intent: {:?}", intent),
        Err(e) => println!("Error creating payment intent: {}", e),
    }
}
