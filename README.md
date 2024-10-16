# Veterinary Service Backend

Welcome to the Veterinary Service Backend! This project is a backend service for managing veterinary appointments, pet information, and owner details. It is built using Rust and the Actix-web framework, with MongoDB as the database.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [API Endpoints](#api-endpoints)
- [Contributing](#contributing)
- [License](#license)

## Features

- Manage bookings for veterinary appointments
- Store and retrieve information about pets (dogs)
- Store and retrieve information about pet owners
- Built with Rust for high performance and safety
- Uses Actix-web for handling HTTP requests
- MongoDB for data storage

## Installation

To get started with the Veterinary Service Backend, follow these steps:

1. **Clone the repository:**
    ```sh
    git clone https://github.com/yourusername/vet-service-backend.git
    cd vet-service-backend
    ```

2. **Set up the environment:**
    Create a `.env` file in the root directory and add your MongoDB URI:
    ```env
    MONGO_URI=your_mongodb_uri
    ```

3. **Build and run the project:**
    ```sh
    cargo build
    cargo run
    ```

## Usage

Once the server is running, you can interact with the API using tools like `curl` or Postman. The server will be running at `http://localhost:8080`.

## API Endpoints

### Booking Routes

- `GET /bookings` - Retrieve all bookings
- `POST /bookings` - Create a new booking
- `DELETE /bookings/{id}` - Cancel a booking

### Dog Routes

- `POST /dog` - Create a new dog entry

### Owner Routes

- `POST /owner` - Create a new owner entry

## Contributing

We welcome contributions! Please fork the repository and submit pull requests.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.