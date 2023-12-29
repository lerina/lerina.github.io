# Demo

[SOURCE](https://blog.ediri.io/serialize-and-deserialize-data-in-rust-using-serde-and-serdejson)  
Note: 
He cut and paste material from `fasterthanlime` so I feel no pb to use this as a quick dev_note on Serde.  
<br/>
We will use the reqwest library to talk to a REST API. 
The reqwest library provides a convenient, high-level client build on the hyper library. 
By default reqwest includes a client that is able of making asynchronous requests. 
But you can also use the reqwest library in a synchronous way 
if you don't want the added complexity of asynchronous code.  
For this demo application, we will use the asynchronous client 
and will need to add tokio to our project. 
tokio is a runtime for asynchronous Rust 🦀 applications.

For this demo application, we will use the [DummyJSON REST API](https://dummyjson.com/).

## Setup

```sh
cargo new demo
cd demo
cargo add reqwest --features="json"
cargo add serde --features="derive"
cargo add tokio --features="full"
```

## Starting point

```rust
#[tokio::main]
async fn main() {
    println!("Hello, world!");
}
```

## with reqwest

We create a new reqwest client and then we make a GET request to the DummyJSON product API. 
We use await to wait for the response and then turn the response into a string also awaiting this operation. 
After all this, we save the string into a variable called product and finally, 
print it to the terminal

```rust
use reqwest::{Client, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let product = Client::new()
        .get("https://dummyjson.com/products/1")
        .send()
        .await?
        .text()
        .await?;
    println!("{:#?}", product);
    Ok(())
}
```

As we used the await method which will return a future, we need to handle possible errors in the main function too. The main function will return a Result type and a reqwest::Error if something goes wrong.

Run it

```
cargo run
```

Output:

```sh
"{\"id\":1,\"title\":\"iPhone 9\",\"description\":\"An apple mobile which is nothing like apple\",\"price\":549,\"discountPercentage\":12.96,\"rating\":4.69,\"stock\":94,\"brand\":\"Apple\",\"category\":\"smartphones\",\"thumbnail\":\"https://i.dummyjson.com/data/products/1/thumbnail.jpg\",\"images\":[\"https://i.dummyjson.com/data/products/1/1.jpg\",\"https://i.dummyjson.com/data/products/1/2.jpg\",\"https://i.dummyjson.com/data/products/1/3.jpg\",\"https://i.dummyjson.com/data/products/1/4.jpg\",\"https://i.dummyjson.com/data/products/1/thumbnail.jpg\"]}"
```


## with Serde

Let's deserialize the JSON data into a Rust 🦀 struct.

To deserialize the response from the DummyJSON API into a Rust 🦀 struct, we need to create a struct called Product that matches the JSON data we get from the API.

The first thing to do is to import the Serialize and Deserialize traits from the serde library:

```
// src/main.rs
...
use serde::{Deserialize, Serialize};
...
```

Now we can create the Product struct and add the Serialize and Deserialize traits to it:

```rust
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub price: i64,
    pub rating: f64,
    pub stock: i64,
    pub brand: String,
    pub category: String,
    pub thumbnail: String,
    pub images: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    ...
}
```


As the API returns the JSON data in camelCase, we need to add the #[serde(rename_all = "camelCase")] attribute to the struct as in Rust 🦀 by convention, the struct fields are in snake case.

Now that we have the Product struct, we can deserialize the JSON data into a Product struct. 
Instead of converting the response to a string, we can convert it directly to a Product struct. 

We do this by using the json method on the response and then awaiting the result. 
The json method will deserialize the JSON data into a Product struct. 
We explicitly annotate the type of the variable product to be a Product struct.


```rust
#[tokio::main]
async fn main() -> Result<(), Error> {
    //let product = Client::new()
    let product: Product = Client::new()
        .get("https://dummyjson.com/products/1")
        .send()
        .await?
        .json()
        .await?;
    println!("{:#?}", product);
    Ok(())
}
```

Run it

```sh
cargo run
```

Output: 

```sh
Product {
    id: 1,
    title: "iPhone 9",
    description: "An apple mobile which is nothing like apple",
    price: 549,
    rating: 4.69,
    stock: 94,
    brand: "Apple",
    category: "smartphones",
    thumbnail: "https://i.dummyjson.com/data/products/1/thumbnail.jpg",
    images: [
        "https://i.dummyjson.com/data/products/1/1.jpg",
        "https://i.dummyjson.com/data/products/1/2.jpg",
        "https://i.dummyjson.com/data/products/1/3.jpg",
        "https://i.dummyjson.com/data/products/1/4.jpg",
        "https://i.dummyjson.com/data/products/1/thumbnail.jpg",
    ],
}
```

Now we can change our code to serialize a Product struct to JSON data and post it to the DummyJSON API create product endpoint.

We need to create a Product struct and fill it with some data. We then need to create a new reqwest::Client but this time we use post instead of get method. We then set the body of the request to JSON and pass a reference to our newly created Product struct. We then await the response and turn the response into json and await the result. The result will then be saved in the new_product variable to finally print it to the terminal.

```rust
// src/main.rs
...

#[tokio::main]
async fn main() -> Result<(), Error> {
    let product: Product = Client::new()
        .get("https://dummyjson.com/products/1")
        .send()
        .await?
        .json()
        .await?;
    println!("{:#?}", product);


    let new_product = Product {
        id: 1,
        title: "Macbook Pro".to_owned(),
        description: "Best laptop ever".to_owned(),
        price: 100,
        rating: 0.0,
        stock: 100,
        brand: "Apple".to_owned(),
        category: "laptops".to_owned(),
        thumbnail: "https://dummyimage.com/300x300/000/fff".to_owned(),
        images: vec![
            "https://dummyimage.com/300x300/000/fff".to_owned(),
            "https://dummyimage.com/600x600/000/fff".to_owned(),
        ],
    };

    let new_product: Product = Client::new()
        .post("https://dummyjson.com/products/add")
        .header("Content-Type", "application/json")
        .json(&new_product)
        .send()
        .await?
        .json()
        .await?;
    println!("{:#?}", new_product);
    Ok(())
}
```

If we run the application now, we should see our new Product successfully created with a new ID (101)

```sh
cargo run
```

Output:

```sh
Product {
    id: 1,
    title: "iPhone 9",
    description: "An apple mobile which is nothing like apple",
    price: 549,
    rating: 4.69,
    stock: 94,
    brand: "Apple",
    category: "smartphones",
    thumbnail: "https://i.dummyjson.com/data/products/1/thumbnail.jpg",
    images: [
        "https://i.dummyjson.com/data/products/1/1.jpg",
        "https://i.dummyjson.com/data/products/1/2.jpg",
        "https://i.dummyjson.com/data/products/1/3.jpg",
        "https://i.dummyjson.com/data/products/1/4.jpg",
        "https://i.dummyjson.com/data/products/1/thumbnail.jpg",
    ],
}
Product {
    id: 101,
    title: "Macbook Pro",
    description: "Best laptop ever",
    price: 100,
    rating: 0.0,
    stock: 100,
    brand: "Apple",
    category: "laptops",
    thumbnail: "https://dummyimage.com/300x300/000/fff",
    images: [
        "https://dummyimage.com/300x300/000/fff",
        "https://dummyimage.com/600x600/000/fff",
    ],
}
```

That works fine, we saw how to serialize and deserialize JSON data in Rust 🦀.

But we are not finished as I want to show is how to handle arbitrary JSON data.

### Handling Arbitrary JSON Data

To start with, add the serde_json crate to our project:

```rust
cargo add serde_json
```

serde_json gives us access to a macro called serde_json::json! which allows using regular JSON syntax to create arbitrary JSON data.

We can add a new reqwest::Client to our application and instead use the Product struct, will switch to the serde_json::json! macro.

We will also change the explicit type annotation of the product variable to `serde_json::Value` instead of the `Product struct`.  
`serde_json::Value` will represent an arbitrary valid JSON value.


```rust
// src/main.rs
...
use serde_json::{json, Value};
...

#[tokio::main]
async fn main() -> Result<(), Error> {
    ...
    //making use of serde_json
    let new_product_arbitary_json: Value = Client::new()
        .post("https://dummyjson.com/products/add")
        .header("Content-Type", "application/json")
        .json(&json!({
            "id": 1,
            "title": "Macbook Pro",
            "description": "Best laptop ever",
            "price": 100,
            "rating": 0.0,
            "stock": 100,
            "brand": "Apple",
            "category": "laptops",
            "thumbnail": "https://dummyimage.com/300x300/000/fff",
            "images": [
                "https://dummyimage.com/300x300/000/fff",
                "https://dummyimage.com/600x600/000/fff"
            ]
        }))
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", new_product_arbitary_json);

    Ok(())
}//^-- main.rs
```

Output:

```sh
...
Product {
    id: 101,
    title: "Macbook Pro",
    description: "Best laptop ever",
    price: 100,
    rating: 0.0,
    stock: 100,
    brand: "Apple",
    category: "laptops",
    thumbnail: "https://dummyimage.com/300x300/000/fff",
    images: [
        "https://dummyimage.com/300x300/000/fff",
        "https://dummyimage.com/600x600/000/fff",
    ],
}
Object {
    "brand": String("Apple"),
    "category": String("laptops"),
    "description": String("Best laptop ever"),
    "id": Number(101),
    "images": Array [
        String("https://dummyimage.com/300x300/000/fff"),
        String("https://dummyimage.com/600x600/000/fff"),
    ],
    "price": Number(100),
    "rating": Number(0),
    "stock": Number(100),
    "thumbnail": String("https://dummyimage.com/300x300/000/fff"),
    "title": String("Macbook Pro"),
}
```

With `serde_json` you will get different `serde_json:from_*` methods to parse JSON data, 
for example, we can use the `serde_json::from_str` function to parse a JSON string into structured data.

```rust
...
#[tokio::main]
async fn main() -> Result<(), Error> {
    ...
    // clone because we are using new_product_arbitary_json again later
    let test: Product = serde_json::from_value(new_product_arbitary_json.clone()).unwrap();
    println!("{:#?}", test);

    Ok(())
}
```

Output:

```sh
...
Object {
    "brand": String("Apple"),
    "category": String("laptops"),
    "description": String("Best laptop ever"),
    "id": Number(101),
    "images": Array [
        String("https://dummyimage.com/300x300/000/fff"),
        String("https://dummyimage.com/600x600/000/fff"),
    ],
    "price": Number(100),
    "rating": Number(0),
    "stock": Number(100),
    "thumbnail": String("https://dummyimage.com/300x300/000/fff"),
    "title": String("Macbook Pro"),
}
Product {
    id: 101,
    title: "Macbook Pro",
    description: "Best laptop ever",
    price: 100,
    rating: 0.0,
    stock: 100,
    brand: "Apple",
    category: "laptops",
    thumbnail: "https://dummyimage.com/300x300/000/fff",
    images: [
        "https://dummyimage.com/300x300/000/fff",
        "https://dummyimage.com/600x600/000/fff",
    ],
}
```

If you have arbitrary JSON data of type serde_json::Value, you can access the data using the Value:get method. This is very similar to HashMap::get method or Vec::get method. The Value::get method will return an Option since the key might not exist in the JSON data.

```rust
#[tokio::main]
async fn main() -> Result<(), Error> {
    ...

    let cloned_product = new_product_arbitary_json.clone();

    let test: Product = serde_json::from_value(new_product_arbitary_json).unwrap();
    println!("{:#?}", test);

    let description = cloned_product.get("description");
    match description {
        Some(d) => println!("Description: {}", d),
        None => println!("No description"),
    }

    Ok(())
}
```

Output:

```sh
...
Product {
    id: 101,
    title: "Macbook Pro",
    description: "Best laptop ever",
    price: 100,
    rating: 0.0,
    stock: 100,
    brand: "Apple",
    category: "laptops",
    thumbnail: "https://dummyimage.com/300x300/000/fff",
    images: [
        "https://dummyimage.com/300x300/000/fff",
        "https://dummyimage.com/600x600/000/fff",
    ],
}
Description: "Best laptop ever"
```

There you have it.
We saw how to use reqwest to make HTTP requests and how to serialize and deserialize JSON data in Rust 🦀 using serde. We also tried out how to handle arbitrary JSON data using serde_json.

