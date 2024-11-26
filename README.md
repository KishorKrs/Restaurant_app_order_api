# Restaurant App Order API

This is a RESTful API for managing orders in a restaurant, built with Rust and SQLite. 
The application allows restaurant staff to add, remove, and query menu items for specific tables.

### Features

- Add menu items to a table with a random cooking time (5-15 minutes).
- Remove menu items from a specific table.
- Query all or specific items for a table.
- Handles 10+ simultaneous requests.
- Designed with functional programming principles in mind.

## Installation

This application need Rust to be installed in the machine If not installed, please install it.
There should be Cargo.toml file in the root of the project.

#### 1. Clone the repository:

```bash
git clone https://github.com/KishorKrs/Restaurant_app_order_api.git
cd Restaurant_app_order_api
```

#### 2. Install dependencies:

```bash
cargo build
```

#### 3. Run the server:

```bash
cargo run
```
This project should be up and running. By default, the server runs on port `8080`, but this can be changed using the PORT environment variable.

## API Endpoints

- POST /orders: Add an item to the menu for a table.
- GET /orders: Query all orders.
- GET /orders/{table_number}: Query orders for a specific table.
- DELETE /orders/{order_id}: Remove an order by its ID.


#### Running Test Case

There is a tests dir in the root of the project. It includes unittest of the api and also util functions. To run test

```bash
cargo test
```

## Contributing

Feel free to fork the repository and submit pull requests with improvements or bug fixes.

## License

[MIT](https://choosealicense.com/licenses/mit/)