<h1 align="center">BOOKERY - API</h1>

![GitHub License](https://img.shields.io/github/license/LucasGoncSilva/bookery?labelColor=101010)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/LucasGoncSilva/bookery/unittest.yml?style=flat&labelColor=%23101010)

Rodando do lado do servidor, a API realiza o processamento dos dados, indo e vindo, para a aplicação Desktop e para o Banco de Dados.
<br><br>
Sua estrutura de arquivos reflete sua estrutura de processamento, suas rotas e interações com o banco de dados. Cada diretório possui arquivos cujo nome de cada arquivo define a qual `struct` - modelo do Banco de Dados - este arquivo trata dentro da lógica definida por seu diretório.

<br>

## Stack

![Rust logo for Axum](https://img.shields.io/badge/Axum-ef4900?style=for-the-badge&logo=rust&logoColor=white)

![PostgreSQL logo](https://img.shields.io/badge/PostgreSQL-316192?style=for-the-badge&logo=postgresql&logoColor=white)

![Docker logo](https://img.shields.io/badge/Docker-2CA5E0?style=for-the-badge&logo=docker&logoColor=white)

<br>

## Arquitetura

A API do Bookery utiliza do Princípio de Responsabilidade Única - Single-Responsability Principle - aplicada ao escopo de arquivos individuais - Single-File Component - de modo que no diretório "images", por exemplo, um arquivo chamado "png.rs" seja um arquivo Rust especializado no processamento de imagens do tipo PNG, apenas. De forma prática pode-se observar a seguinte estrutura:

```bash
.
├── Cargo.toml                                # Arquivo de dependências do projeto
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

![Arquitetura Geral](./arch_api_detailed.svg)

O fluxo acima ocorre - na API - todo a partir do `main.rs`, podendo interpretá-lo como a própria box de "APP" no esquema acima, visto que tudo quando compilado é estruturado e organizado através da variável `app`, na declaração `let app: Router = router::router(db);` dentro do arquivo informado anteriormente.

<br>

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

<br>

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
            <td rowspan=6><pre>Author</pre></td>
            <td>Criar</td>
            <td>POST</td>
            <td>/author/create</td>
        </tr>
        <tr>
            <td>Obter</td>
            <td>GET</td>
            <td><code>/author/get/:id</code></td>
        </tr>
        <tr>
            <td>Filtrar</td>
            <td>GET</td>
            <td>/author/search</td>
        </tr>
        <tr>
            <td>Atualizar</td>
            <td>POST</td>
            <td>/author/update</td>
        </tr>
        <tr>
            <td>Deletar</td>
            <td>POST</td>
            <td>/author/delete</td>
        </tr>
        <tr>
            <td>Contar</td>
            <td>GET</td>
            <td>/author/count</td>
        </tr>
        <tr>
            <td rowspan=8><code>Book</code></td>
            <td>Criar</td>
            <td>POST</td>
            <td>/book/create</td>
        </tr>
        <tr>
            <td>Obter</td>
            <td>GET</td>
            <td>/book/get/:id</td>
        </tr>
        <tr>
            <td>Obter Bruto</td>
            <td>GET</td>
            <td>/book/get-raw/:id</td>
        </tr>
        <tr>
            <td>Filtrar</td>
            <td>GET</td>
            <td>/book/search</td>
        <tr>
            <td>Filtrar Bruto</td>
            <td>GET</td>
            <td>/book/search-raw</td>
        </tr>
        <tr>
            <td>Atualizar</td>
            <td>POST</td>
            <td>/book/update</td>
        </tr>
        <tr>
            <td>Deletar</td>
            <td>POST</td>
            <td>/book/delete</td>
        </tr>
        <tr>
            <td>Contar</td>
            <td>GET</td>
            <td>/book/count</td>
        </tr>
        <tr>
            <td rowspan=6><code>Costumer</code></td>
            <td>Criar</td>
            <td>POST</td>
            <td>/costumer/create</td>
        </tr>
        <tr>
            <td>Obter</td>
            <td>GET</td>
            <td>/costumer/get/:id</td>
        </tr>
        <tr>
            <td>Filtrar</td>
            <td>GET</td>
            <td>/costumer/search</td>
        </tr>
        <tr>
            <td>Atualizar</td>
            <td>POST</td>
            <td>/costumer/update</td>
        </tr>
        <tr>
            <td>Deletar</td>
            <td>POST</td>
            <td>/costumer/delete</td>
        </tr>
        <tr>
            <td>Contar</td>
            <td>GET</td>
            <td>/costumer/count</td>
        </tr>
        <tr>
            <td rowspan=8><code>Rental</code></td>
            <td>Criar</td>
            <td>POST</td>
            <td>/rental/create</td>
        </tr>
        <tr>
            <td>Obter</td>
            <td>GET</td>
            <td>/rental/get/:id</td>
        </tr>
        <tr>
            <td>Obter Bruto</td>
            <td>GET</td>
            <td>/rental/get-raw/:id</td>
        </tr>
        <tr>
            <td>Filtrar</td>
            <td>GET</td>
            <td>/rental/search</td>
        <tr>
            <td>Filtrar Bruto</td>
            <td>GET</td>
            <td>/rental/search-raw</td>
        </tr>
        <tr>
            <td>Atualizar</td>
            <td>POST</td>
            <td>/rental/update</td>
        </tr>
        <tr>
            <td>Deletar</td>
            <td>POST</td>
            <td>/rental/delete</td>
        </tr>
        <tr>
            <td>Contar</td>
            <td>GET</td>
            <td>/rental/count</td>
        </tr>
    </tbody>
</table>
