<h1 align="center">
  <img src="./logo.svg" height="300" width="300" alt="Logo BOOKERY" /><br>
  BOOKERY
</h1>

![GitHub License](https://img.shields.io/github/license/LucasGoncSilva/bookery?labelColor=101010)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/LucasGoncSilva/bookery/unittest.yml?style=flat&labelColor=%23101010)

Built on Axum and Tauri, Rust frameworks for API and Desktop, respectively, Bookery is a mini desktop system for libraries to manage their books and loans.

Bookery allows you to efficiently register and search for authors, books, customers and rentals, offering a complete management experience with advanced filters, editing options and an intuitive interface. Each feature has been designed to ensure that libraries can manage their collections and transactions quickly and accurately.

Developed with a rigorous focus on quality, the system is evaluated by more than 140 automated tests, guaranteeing robustness and reliability. Taking advantage of Rust's efficiency, Bookery outperforms solutions such as Electron in terms of processing and memory usage, while the Axum API provides performance perfectly comparable to C/C++, offering high performance with simplicity and efficiency.

## Stack

![Tauri logo](https://img.shields.io/badge/Tauri-0f0f0f?style=for-the-badge&logo=Tauri&logoColor=f7bb2f)

![HTML logo](https://img.shields.io/badge/HTML5-E34F26?style=for-the-badge&logo=html5&logoColor=white)
![CSS logo](https://img.shields.io/badge/CSS3-1572B6?style=for-the-badge&logo=css3&logoColor=white)
![Sass logo](https://img.shields.io/badge/Sass-CC6699?style=for-the-badge&logo=sass&logoColor=white)
![JavaScript logo](https://img.shields.io/badge/JavaScript-323330?style=for-the-badge&logo=javascript&logoColor=F7DF1E)

![Rust logo for Axum](https://img.shields.io/badge/Axum-ef4900?style=for-the-badge&logo=rust&logoColor=white)

![PostgreSQL logo](https://img.shields.io/badge/PostgreSQL-316192?style=for-the-badge&logo=postgresql&logoColor=white)

![Docker logo](https://img.shields.io/badge/Docker-2CA5E0?style=for-the-badge&logo=docker&logoColor=white)

## Arch

The architecture can be broadly broken down into two fronts: Desktop and API. Treating the Desktop application as the client of this solution, a behavioral view will be detailed on a “macro” scale, following the flow of data without focusing on the “micro”, such as each action of each function. Follow the general flow of information below:

```mermaid
flowchart RL


subgraph CLOUD
    subgraph APP
        API{{API}}:::Arch
    end

    subgraph PERSISTENCE
        Database[(Database)]:::Arch
    end
end

Desktop[Desktop]


Desktop --> API
API --> Database
Database --> API
API --> Desktop


style CLOUD fill:#ccc7,color:#800,stroke:#800;
style APP fill:#ccc7,color:#800,stroke:#800;
style PERSISTENCE fill:#ccc7,color:#800,stroke:#800;

style Desktop fill:#800000,color:#fff,stroke:#fff;

classDef Arch fill:#800,color:#efe,stroke:#efe;

linkStyle 0,1 stroke:#f00
linkStyle default stroke:#800
```

## Details

For organizational reasons, in order to maintain consistency in the Bookery documentation, the details of each side of the system are described within their own modules. Consider visiting the addresses below to view the details of each module's architecture:

- [Arquitetura Desktop](https://github.com/LucasGoncSilva/bookery/tree/main/BOOKERY/desktop) - Client of the solution, part that runs on the user's machine
- [Arquitetura API](https://github.com/LucasGoncSilva/bookery/tree/main/BOOKERY/api) - Solution server, side maintained in the cloud
- [Arquitetura Compartilhada](https://github.com/LucasGoncSilva/bookery/tree/main/BOOKERY/shared) - Hub for shared structures between the aforementioned modules

## License

This project is under [MPLv2 - Mozilla Public License Version 2.0](https://choosealicense.com/licenses/mpl-2.0/). Permissions of this weak copyleft license are conditioned on making available source code of licensed files and modifications of those files under the same license (or in certain cases, one of the GNU licenses). Copyright and license notices must be preserved. Contributors provide an express grant of patent rights. However, a larger work using the licensed work may be distributed under different terms and without source code for files added in the larger work.
