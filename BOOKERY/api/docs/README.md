<h1 align="center">BOOKERY - API</h1>

![GitHub License](https://img.shields.io/github/license/LucasGoncSilva/bookery?labelColor=101010)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/LucasGoncSilva/bookery/unittest.yml?style=flat&labelColor=%23101010)

Running on the server side, the API processes data back and forth to the desktop application and the database.

Its file structure reflects its processing structure, its routes and interactions with the database. Each directory has files whose filename defines which `struct` - database model - this file deals with within the logic defined by its directory.

## Stack

![Rust logo for Axum](https://img.shields.io/badge/Axum-ef4900?style=for-the-badge&logo=rust&logoColor=white)

![PostgreSQL logo](https://img.shields.io/badge/PostgreSQL-316192?style=for-the-badge&logo=postgresql&logoColor=white)

![Docker logo](https://img.shields.io/badge/Docker-2CA5E0?style=for-the-badge&logo=docker&logoColor=white)

## Arch

The Bookery API uses the Single-Responsability Principle applied to the scope of individual files - Single-File Component - so that in the “images” directory, for example, a file called “png.rs” is a Rust file specialized in processing PNG-type images only. In practical terms, you can see the following structure:

```bash
.
├── Cargo.toml                                          # Project dependencies file
│
├── docs                                                # Documentation directory
│   ├── README.md                                       # Main reading file
│   └── ...                                             # Other files useful for documentation
│
└── src                                                 # Source code directory
    │
    ├── database                                        # Database responsibilities directory
    │   ├── mod.rs                                      # Directory modularization file
    │   ├── conn.rs                                     # File responsible for connecting to the Database
    │   ├── author.rs                                   # Specialist file on “Author” struct
    │   ├── book.rs                                     # Specialist file on “Book” struct
    │   ├── costumer.rs                                 # Specialist file on “Costumer” struct
    │   └── rental.rs                                   # Specialist file on “Rental” struct
    │
    ├── handlers                                        # Directory of processing function responsibilities
    │   ├── mod.rs                                      # Directory modularization file
    │   ├── author.rs                                   # Specialist file on “Author” struct
    │   ├── book.rs                                     # Specialist file on “Book” struct
    │   ├── costumer.rs                                 # Specialist file on “Costumer” struct
    │   └── rental.rs                                   # Specialist file on “Rental” struct
    │
    ├── router.rs                                       # File for defining routes and methods
    │
    ├── migrations                                      # Directory related to database migrations
    │   └── 0000_create_table_example.sql               # Individual database migrations in sequence
    │
    └── main.rs                                         # Project input file - API
```

**NOTE: Just to inform you that there is no `struct` directory or file listed above because the `Author`, `Book`, `Costumer` and `Rental` structures have been defined within the workspace in the directory named `shared`. This arrangement is due to the fact that the structures mentioned above are shared between the two fronts of the project, used both on the Desktop and in the API.**

The API architecture seen in detail, with the Desktop as the client and accessing the Database, still on a macro scale but looking in more detail at the application's Back-end, we then have the following situation:

```mermaid
flowchart BT


subgraph CLOUD
    subgraph APP
        Routes{Routes}:::Arch

        subgraph "/handlers"
            AuthorH(["Author"]):::Arch
            BookH(["Book"]):::Arch
            CostumerH(["Costumer"]):::Arch
            RentalH(["Rental"]):::Arch
        end

        subgraph "/database"
            AuthorD(["Author"]):::Arch
            BookD(["Book"]):::Arch
            CostumerD(["Costumer"]):::Arch
            RentalD(["Rental"]):::Arch
        end
    end

    subgraph PERSISTENCE
        Persistence[(Database)]:::Arch
    end
end

Desktop[Desktop]


Desktop <--> Routes

Routes <--> AuthorH & BookH & CostumerH & RentalH

AuthorH <--> AuthorD
BookH <--> BookD
CostumerH <--> CostumerD
RentalH <--> RentalD

AuthorD & BookD & CostumerD & RentalD <--> Persistence


style CLOUD fill:#ccc7,color:#800,stroke:#800;
style APP fill:#ccc7,color:#800,stroke:#800;
style PERSISTENCE fill:#ccc7,color:#800,stroke:#800;
style /handlers fill:#8007,color:#fff,stroke:#fff;
style /database fill:#8007,color:#fff,stroke:#fff;

style Desktop fill:#800000,color:#fff,stroke:#fff;

classDef Arch fill:#800,color:#efe,stroke:#efe;

linkStyle default stroke:#800
```

The above flow takes place - in the API - all from `main.rs`, and we can interpret it as the “APP” box itself in the above schema, since everything when compiled is structured and organized through the `app` variable, in the `let app: Router = router::router(db);` statement inside the file mentioned above.

## Basic

Before starting with development and commands, it is important to define the environment variables in your development environment. Below is a list of which ones to set:

| Name           | Type     | Mandatory  |                         Default                          | Desc                           |
| :------------- | :------- | :--------: | :------------------------------------------------------: | :----------------------------- |
| `DATABASE_URL` | `String` | `Optional` | `"postgres://postgres:postgres@localhost:5432/postgres"` | Database connection string/URL |

### Run Automated Tests

`cargo test`

### Run Local Server

`cargo run` para desenvolvimento

`cargo run --release` para performance de produção

## Endpoints

The API routes are divided between each `struct` and organized by actions, as well as being divided by the structures themselves, of course; follow the organization below for more details:

<table>
    <thead>
        <tr>
            <th>Struct</th>
            <th>Action</th>
            <th>Method</th>
            <th>Endpoint</th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td rowspan=6>Author</td>
            <td>Create</td>
            <td>POST</td>
            <td><code>/author/create</code></td>
        </tr>
        <tr>
            <td>Get</td>
            <td>GET</td>
            <td><code>/author/get/:id</code></td>
        </tr>
        <tr>
            <td>Filter</td>
            <td>GET</td>
            <td><code>/author/search</code></td>
        </tr>
        <tr>
            <td>Update</td>
            <td>POST</td>
            <td><code>/author/update</code></td>
        </tr>
        <tr>
            <td>Delete</td>
            <td>POST</td>
            <td><code>/author/delete</code></td>
        </tr>
        <tr>
            <td>Count</td>
            <td>GET</td>
            <td><code>/author/count</code></td>
        </tr>
        <tr>
            <td rowspan=8>Book</td>
            <td>Create</td>
            <td>POST</td>
            <td><code>/book/create</code></td>
        </tr>
        <tr>
            <td>Get</td>
            <td>GET</td>
            <td><code>/book/get/:id</code></td>
        </tr>
        <tr>
            <td>Get Raw</td>
            <td>GET</td>
            <td><code>/book/get-raw/:id</code></td>
        </tr>
        <tr>
            <td>Filter</td>
            <td>GET</td>
            <td><code>/book/search</code></td>
        </tr>
            <td>Filter Raw</td>
            <td>GET</td>
            <td><code>/book/search-raw</code></td>
        </tr>
        <tr>
            <td>Update</td>
            <td>POST</td>
            <td><code>/book/update</code></td>
        </tr>
        <tr>
            <td>Delete</td>
            <td>POST</td>
            <td><code>/book/delete</code></td>
        </tr>
        <tr>
            <td>Count</td>
            <td>GET</td>
            <td><code>/book/count</code></td>
        </tr>
        <tr>
            <td rowspan=6>Costumer</td>
            <td>Create</td>
            <td>POST</td>
            <td><code>/costumer/create</code></td>
        </tr>
        <tr>
            <td>Get</td>
            <td>GET</td>
            <td><code>/costumer/get/:id</code></td>
        </tr>
        <tr>
            <td>Filter</td>
            <td>GET</td>
            <td><code>/costumer/search</code></td>
        </tr>
        <tr>
            <td>Update</td>
            <td>POST</td>
            <td><code>/costumer/update</code></td>
        </tr>
        <tr>
            <td>Delete</td>
            <td>POST</td>
            <td><code>/costumer/delete</code></td>
        </tr>
        <tr>
            <td>Count</td>
            <td>GET</td>
            <td><code>/costumer/count</code></td>
        </tr>
        <tr>
            <td rowspan=8>Rental</td>
            <td>Create</td>
            <td>POST</td>
            <td><code>/rental/create</code></td>
        </tr>
        <tr>
            <td>Get</td>
            <td>GET</td>
            <td><code>/rental/get/:id</code></td>
        </tr>
        <tr>
            <td>Get Raw</td>
            <td>GET</td>
            <td><code>/rental/get-raw/:id</code></td>
        </tr>
        <tr>
            <td>Filter</td>
            <td>GET</td>
            <td><code>/rental/search</code></td>
        </tr>
            <td>Filter Raw</td>
            <td>GET</td>
            <td><code>/rental/search-raw</code></td>
        </tr>
        <tr>
            <td>Update</td>
            <td>POST</td>
            <td><code>/rental/update</code></td>
        </tr>
        <tr>
            <td>Delete</td>
            <td>POST</td>
            <td><code>/rental/delete</code></td>
        </tr>
        <tr>
            <td>Count</td>
            <td>GET</td>
            <td><code>/rental/count</code></td>
        </tr>
    </tbody>
</table>
