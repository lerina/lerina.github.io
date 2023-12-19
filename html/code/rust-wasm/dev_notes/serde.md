# The Serde framework

Serde is a framework for serializing and deserializing Rust data structures efficiently and generically. For this section of the article, we will make use of the serde crate to parse a JSON file and a TOML file.

The basic advantage of the serde library is that it allows you to directly parse wire data into Rust structs that match the types defined within our source code. This way, your project knows the expected type of every incoming data at compile time of the source code.

## Reading a JSON file

The JSON format is a popular data format for storing complex data. It’s the predominant data format amongst the common data formats used to exchange wire data on the web. It is widely used across JavaScript projects.

We can approach parsing of JSON data in Rust through a statically-typed method and a dynamically-typed method.

The dynamically-typed method is best suited for cases where you’re uncertain of the format of the JSON data against your predefined data structure in your source code, while the statically-typed method is used when you’re certain of the format of the JSON data.

To get started, you must install all the required dependencies.

In the Cargo.toml file, we’ll first add the serde and the serde_json crates as the dependencies. In addition to this, make sure the optional derive feature is enabled, which will help us generate the code for the (de)serialization.


```toml
#Cargo.toml

[dependencies]
serde = { version = 1.0, features = [“derived”] }
serde_json = "1.0"
```

