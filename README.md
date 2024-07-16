# Haberlendir

Haberlendir is an application that reads RSS feeds and transforms them into an API using Axum and MongoDB.

## Features

- Fetches and parses RSS feeds from various sources.
- Stores parsed data in a MongoDB database.
- Provides a RESTful API to access the news data.
- Built with Axum for the API layer.

## Installation

To install and run the project, follow these steps:

1. **Clone the repository:**
    ```bash
    git clone https://github.com/ice777x/haberlendir.git
    cd haberlendir
    ```

2. **Build the project:**
    ```bash
    cargo build --release
    ```

3. **Set up your environment variables:**
    Create a `.env` file in the root directory and add the following:
    ```plaintext
    MONGO_URI=your_mongodb_connection_string
    AUTH_TOKEN=your_authentication_token
    ```

4. **Run the project:**
    ```bash
    cargo run
    ```

## Usage

Once the project is running, you can use the following endpoints to interact with the API:

- `GET /api/find` - Fetches the latest news from all RSS feeds.
- `GET /api/resourcers` - List of RSS providers.
- `DELETE /api/delete` - Deletes feeds based on given parameters.

### Search Parameters for `/api/find`

The `/api/find` endpoint supports the following query parameters for filtering results:

- `author` (bool) - Filter news by author.
- `q` (query) - Search query for news titles or content.
- `limit` (i64) - Limit the number of results returned.
- `skip` (u64) - Skip a number of results.

### Delete Parameters for `/api/delete`

The `/api/delete` endpoint supports the following JSON parameters for deleting data:

- `ids` (string[]) - Array of IDs to delete. That is optional.
- `all` (bool) - If set to `true`, deletes all data.

### Authentication

Authentication is required for the `/api/delete` endpoint. Include an `Authorization` header with the value `your_authentication_token` where `your_authentication_token` is obtained from the `.env` file.

### Example Requests

- **Get all news:**
    ```bash
    curl http://localhost:42069/api/find
    ```

- **Get news with parameters:**
    ```bash
    curl "http://localhost:42069/api/find?author=false&q=tech&limit=10&skip=5"
    ```

- **Get a specific news article:**
    ```bash
    curl http://localhost:42069/api/find/<id>
    ```

- **Get list of RSS providers:**
    ```bash
    curl http://localhost:42069/api/resourcers
    ```

- **Delete specific feeds:**
    ```bash
    curl -X DELETE -H "Authorization: your_authentication_token" -H "Content-Type: application/json" -d '{"ids":[1,2,3]}' http://localhost:42069/api/delete
    ```

- **Delete all feeds:**
    ```bash
    curl -X DELETE -H "Authorization: your_authentication_token" -H "Content-Type: application/json" -d '{"all":true}' http://localhost:42069/api/delete
    ```

## Contributing

Contributions are welcome! If you have any ideas, suggestions, or bug reports, please create an issue or submit a pull request.

1. Fork the repository.
2. Create your feature branch (`git checkout -b feature/your-feature`).
3. Commit your changes (`git commit -m 'Add some feature'`).
4. Push to the branch (`git push origin feature/your-feature`).
5. Open a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

## Acknowledgements

- [Axum](https://github.com/tokio-rs/axum) - The web framework used for building the API.
- [MongoDB](https://www.mongodb.com/) - The database used for storing news data.
- [RSS](https://en.wikipedia.org/wiki/RSS) - The format used for reading news feeds.
