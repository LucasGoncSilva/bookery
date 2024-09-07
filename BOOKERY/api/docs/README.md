<h1 align="center">BOOKERY - API</h1>

![GitHub License](https://img.shields.io/github/license/LucasGoncSilva/bookery?labelColor=101010)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/LucasGoncSilva/bookery/unittest.yml?style=flat&labelColor=%23101010)

Rodando do lado do servidor, a API realiza o processamento dos dados, indo e vindo, para a aplicação Desktop e para o Banco de Dados.

Sua estrutura de arquivos reflete sua estrutura de processamento, suas rotas e interações com o banco de dados. Cada diretório possui arquivos cujo nome de cada arquivo define a qual `struct` - modelo do Banco de Dados - este arquivo trata dentro da lógica definida por seu diretório.

## Stack

![Rust logo for Axum](https://img.shields.io/badge/Axum-ef4900?style=for-the-badge&logo=rust&logoColor=white)

![PostgreSQL logo](https://img.shields.io/badge/PostgreSQL-316192?style=for-the-badge&logo=postgresql&logoColor=white)

![Docker logo](https://img.shields.io/badge/Docker-2CA5E0?style=for-the-badge&logo=docker&logoColor=white)

## Arquitetura

A API do Bookery utiliza do Princípio de Responsabilidade Única - Single-Responsability Principle - aplicada ao escopo de arquivos individuais - Single-File Component - de modo que no diretório "images", por exemplo, um arquivo chamado "png.rs" seja um arquivo Rust especializado no processamento de imagens do tipo PNG, apenas. De forma prática pode-se observar a seguinte estrutura:

```bash
.
├── Cargo.toml                                # Arquivo de dependências do projeto
│
├── docs                                      # Diretório da documentação
│   ├── README.md                             # Arquivo principal de leitura
│   └── ...                                   # Demais arquivos úteis para documentação
│
└── src                                       # Diretório do código-fonte
    │
    ├── database                              # Diretório de responsabilidades do Banco de Dados
    │   ├── mod.rs                            # Arquivo de modularização do diretório
    │   ├── conn.rs                           # Arquivo responsável na conexão com o Banco de Dados
    │   ├── author.rs                         # Arquivo especialista na struct "Author"
    │   ├── book.rs                           # Arquivo especialista na struct "Book"
    │   ├── costumer.rs                       # Arquivo especialista na struct "Costumer"
    │   └── rental.rs                         # Arquivo especialista na struct "Rental"
    │
    ├── handlers                              # Diretório de responsabilidades das funções de processamento
    │   ├── mod.rs                            # Arquivo de modularização do diretório
    │   ├── author.rs                         # Arquivo especialista na struct "Author"
    │   ├── book.rs                           # Arquivo especialista na struct "Book"
    │   ├── costumer.rs                       # Arquivo especialista na struct "Costumer"
    │   └── rental.rs                         # Arquivo especialista na struct "Rental"
    │
    ├── router.rs                             # Arquivo de definição de rotas e métodos
    │
    ├── migrations                            # Diretório relacionado às migrações do Banco de Dados
    │   └── 0000_create_table_example.sql     # Migrações individuais do Banco de Dados em sequência
    │
    └── main.rs                               # Arquivo de entrada do projeto - API
```

**OBS: apenas informando que não há nenhum diretório ou arquivo de `struct` listado acima pois as estruturas `Author`, `Book`, `Costumer` e `Rental` foram definidas dentro do workspace no diretório de nome `shared`. Esta disposição se deve ao fato de que as estruturas citadas anteriormente são compartilhadas entre as duas frentes do projeto, utilizadas tanto no Desktop quanto na API.**

A arquitetura da API vista em detalhes, tendo o Desktop como cliente e acessando o Banco de Dados, ainda em escala macro mas observando com mais detalhes o Back-end da aplicação, temos então a seguinte situação:

<!-- ![Arquitetura Geral](./arch.svg) -->

```mermaid
flowchart TB


subgraph CLOUD
    subgraph PERSISTENCE
        Database[(Database)]:::Arch
    end

    subgraph APP
        API{{API}}:::Arch
    end
end

subgraph DESKTOP
    HTML(index.html):::Arch

    CSS[[styles.css]]:::Arch
    SCSS[[styles.scss]]:::Arch
    CSSMAP[[styles.css.map]]:::Arch

    JS[[main.js]]:::Arch

    RS(main.rs):::Arch
end

User(((User)))


User --> HTML

SCSS & CSSMAP -.-> CSS
JS & CSS -.-> HTML

HTML --> RS

RS ~~~ API
RS --> API
API --> Database
Database --> API
Database ~~~ API
Database ~~~ API
API --> RS

RS --> HTML
HTML --> User


style CLOUD fill:#ccc7,color:#800,stroke:#800;
style APP fill:#ccc7,color:#800,stroke:#800;
style PERSISTENCE fill:#ccc7,color:#800,stroke:#800;
style DESKTOP fill:#ccc7,color:#800,stroke:#800;

style User fill:#fff,color:#800,stroke:#800;

classDef Arch fill:#800,color:#efe,stroke:#efe;

linkStyle 1,2,3,4 stroke:#fff
linkStyle 5,6,7,8,0 stroke:#f00
linkStyle default stroke:#800
```

O fluxo acima ocorre - na API - todo a partir do `main.rs`, podendo interpretá-lo como a própria box de "APP" no esquema acima, visto que tudo quando compilado é estruturado e organizado através da variável `app`, na declaração `let app: Router = router::router(db);` dentro do arquivo informado anteriormente.

## Básico

Antes de iniciar com o desenvolvimento e os comandos, é importante definir as variáveis de ambiente no seu ambiente de desenvolvimento. Abaixo a listagem de quais definir:

| Variável       | Caráter             | Responsabilidade                                                                                           |
| :------------- | :------------------ | :--------------------------------------------------------------------------------------------------------- |
| `DATABASE_URL` | `String - optional` | String de conexão com o Banco de Dados<br>Default `"postgres://postgres:postgres@localhost:5432/postgres"` |

### Iniciar Testes Automatizados

`cargo test`

### Iniciar Servidor

`cargo run` para desenvolvimento

`cargo run --release` para performance de produção

## Endpoints

As rotas da API se dividem entre cada `struct` organizando-se por ações, além de se dividir através das próprias estruturas, é claro; acompanhe abaixo a organização para mais detalhes:

<table>
    <thead>
        <tr>
            <th>Struct</th>
            <th>Ação</th>
            <th>Método</th>
            <th>Endpoint</th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td rowspan=6>Author</td>
            <td>Criar</td>
            <td>POST</td>
            <td><code>/author/create</code></td>
        </tr>
        <tr>
            <td>Obter</td>
            <td>GET</td>
            <td><code>/author/get/:id</code></td>
        </tr>
        <tr>
            <td>Filtrar</td>
            <td>GET</td>
            <td><code>/author/search</code></td>
        </tr>
        <tr>
            <td>Atualizar</td>
            <td>POST</td>
            <td><code>/author/update</code></td>
        </tr>
        <tr>
            <td>Deletar</td>
            <td>POST</td>
            <td><code>/author/delete</code></td>
        </tr>
        <tr>
            <td>Contar</td>
            <td>GET</td>
            <td><code>/author/count</code></td>
        </tr>
        <tr>
            <td rowspan=8>Book</td>
            <td>Criar</td>
            <td>POST</td>
            <td><code>/book/create</code></td>
        </tr>
        <tr>
            <td>Obter</td>
            <td>GET</td>
            <td><code>/book/get/:id</code></td>
        </tr>
        <tr>
            <td>Obter Bruto</td>
            <td>GET</td>
            <td><code>/book/get-raw/:id</code></td>
        </tr>
        <tr>
            <td>Filtrar</td>
            <td>GET</td>
            <td><code>/book/search</code></td>
        </tr>
            <td>Filtrar Bruto</td>
            <td>GET</td>
            <td><code>/book/search-raw</code></td>
        </tr>
        <tr>
            <td>Atualizar</td>
            <td>POST</td>
            <td><code>/book/update</code></td>
        </tr>
        <tr>
            <td>Deletar</td>
            <td>POST</td>
            <td><code>/book/delete</code></td>
        </tr>
        <tr>
            <td>Contar</td>
            <td>GET</td>
            <td><code>/book/count</code></td>
        </tr>
        <tr>
            <td rowspan=6>Costumer</td>
            <td>Criar</td>
            <td>POST</td>
            <td><code>/costumer/create</code></td>
        </tr>
        <tr>
            <td>Obter</td>
            <td>GET</td>
            <td><code>/costumer/get/:id</code></td>
        </tr>
        <tr>
            <td>Filtrar</td>
            <td>GET</td>
            <td><code>/costumer/search</code></td>
        </tr>
        <tr>
            <td>Atualizar</td>
            <td>POST</td>
            <td><code>/costumer/update</code></td>
        </tr>
        <tr>
            <td>Deletar</td>
            <td>POST</td>
            <td><code>/costumer/delete</code></td>
        </tr>
        <tr>
            <td>Contar</td>
            <td>GET</td>
            <td><code>/costumer/count</code></td>
        </tr>
        <tr>
            <td rowspan=8>Rental</td>
            <td>Criar</td>
            <td>POST</td>
            <td><code>/rental/create</code></td>
        </tr>
        <tr>
            <td>Obter</td>
            <td>GET</td>
            <td><code>/rental/get/:id</code></td>
        </tr>
        <tr>
            <td>Obter Bruto</td>
            <td>GET</td>
            <td><code>/rental/get-raw/:id</code></td>
        </tr>
        <tr>
            <td>Filtrar</td>
            <td>GET</td>
            <td><code>/rental/search</code></td>
        </tr>
            <td>Filtrar Bruto</td>
            <td>GET</td>
            <td><code>/rental/search-raw</code></td>
        </tr>
        <tr>
            <td>Atualizar</td>
            <td>POST</td>
            <td><code>/rental/update</code></td>
        </tr>
        <tr>
            <td>Deletar</td>
            <td>POST</td>
            <td><code>/rental/delete</code></td>
        </tr>
        <tr>
            <td>Contar</td>
            <td>GET</td>
            <td><code>/rental/count</code></td>
        </tr>
    </tbody>
</table>
