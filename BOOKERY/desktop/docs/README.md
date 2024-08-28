<h1 align="center">BOOKERY - Desktop</h1>

![GitHub License](https://img.shields.io/github/license/LucasGoncSilva/bookery?labelColor=101010)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/LucasGoncSilva/bookery/unittest.yml?style=flat&labelColor=%23101010)

Rodando do lado do cliente, literalmente, o Desktop se responsabilida da comunicação direta com o usuário final do sistema. É o canal de contato entre o operador do sistema e todas as informações que o sistema apresenta.

Sua estrutura de arquivos reflete sua estrutura de processamento, suas rotas e interações com o banco de dados. Cada diretório possui arquivos cujo nome de cada arquivo define a qual `struct` - modelo do Banco de Dados - este arquivo trata dentro da lógica definida por seu diretório.

## Stack

![Tauri logo](https://img.shields.io/badge/Tauri-0f0f0f?style=for-the-badge&logo=Tauri&logoColor=f7bb2f)

![HTML logo](https://img.shields.io/badge/HTML5-E34F26?style=for-the-badge&logo=html5&logoColor=white)
![CSS logo](https://img.shields.io/badge/CSS3-1572B6?style=for-the-badge&logo=css3&logoColor=white)
![Sass logo](https://img.shields.io/badge/Sass-CC6699?style=for-the-badge&logo=sass&logoColor=white)
![JavaScript logo](https://img.shields.io/badge/JavaScript-323330?style=for-the-badge&logo=javascript&logoColor=F7DF1E)

## Arquitetura

O Desktop do Bookery apresenta a arquitetura padrão de um projeto Tauri, definida pelo próprio Framework. De forma prática pode-se observar a seguinte estrutura:

```bash
.
├── docs                                      # Diretório da documentação
│   ├── README.md                             # Arquivo principal de leitura
│   └── ...                                   # Demais arquivos úteis para documentação
│
├── package.json                              # Arquivo de gerenciamento de dependências
├── package-lock.json                         # Arquivo de gerenciamento de dependências
│
├── src                                       # Diretório do código-fonte das interfaces do usuário
│   ├── index.html                            # Arquivo de entrada das interfaces
│   ├── main.js                               # Arquivo principal de comportamentos em JavaScript
│   ├── styles.css                            # Arquivo de estilos autogerado e comprimido para otimização
│   ├── styles.css.map                        # Arquivo de config e organização para o arquivo acima
│   └── styles.scss                           # Arquivo de estilos definidos
│
└── src-tauri                                 # Diretório do código-fonte do compilador/bundle
    ├── build.rs                              # Arquivo de config da build
    │
    ├── Cargo.toml                            # Arquivo de dependências do projeto
    │
    ├── icons                                 # Diretório de ícones da aplicação
    │   ├── *.icns                            # Arquivos de ícones no format .icns
    │   ├── *.ico                             # Arquivos de ícones no format .ico
    │   └── *.png                             # Arquivos de ícones no format .icns
    │
    ├── tauri.conf.json                       # Arquivo de auxílio e configuração de biuld
    │
    └── src                                   # Diretório do código-fonte do motor
        └── main.rs                           # Arquivo de entrada do "Back-end" do executável
```

**OBS: apenas informando que não há nenhum diretório ou arquivo de `struct` listado acima pois as estruturas `Author`, `Book`, `Costumer` e `Rental` foram definidas dentro do workspace no diretório de nome `shared`. Esta disposição se deve ao fato de que as estruturas citadas anteriormente são compartilhadas entre as duas frentes do projeto, utilizadas tanto no Desktop quanto na API.**

A arquitetura do Desktop vista em detalhes, tendo a API como servidor que, por sua vez, conforme necessidade acessa o Banco de Dados; ainda em escala macro mas observando com mais detalhes o Front-end da aplicação, temos então a seguinte situação:

![Arquitetura Geral](./arch.svg)

O fluxo acima ocorre - no Desktop - todo agrupado, compilado e gerado em um único bundle executável (`.exe` no Windows, `.app` no MacOS, AppImage no Linux), sendo, portanto, tudo "uma coisa só" - entre muitas aspas. Ainda que seja tudo uma única coisa, internamente as responsabilidades e ordem lógica é estabelecida como indicado no diagrama acima.

## Básico

Antes de iniciar com o desenvolvimento e os comandos, é importante definir as variáveis de ambiente no seu ambiente de desenvolvimento. Abaixo a listagem de quais definir:

| Variável  | Caráter           | Responsabilidade                                                                              |
| :-------- | :---------------- | :-------------------------------------------------------------------------------------------- |
| `API_URL` | `&str - required` | String de conexão com a API<br>Default `postgres://postgres:postgres@localhost:5432/postgres` |

<!-- ### Iniciar Testes Automatizados

`cargo test` -->

### Iniciar Servidor

`npm run tauri dev`

### Criar Bundle

`npm run tauri build`

## Distribuição de Tarefas: JS e Rust

No mini ecossistema fechado denominado Bookery, algumas tarefas administrativas só podem ser realizadas pelo JavaScript, outras apenas pelo Rust, contudo, algumas delas poderiam ser realizadas por ambos. Aqui são destacadas e definidas com mais detalhes quais as responsabilidades de cada linguagem em determinado tipo de tarefa, juntamente com a justificativa de cada escolha.

### JavaScript

As tarefas de responsabilidade única e exclusivamente do JS são aquelas que interagem e impactam diretamente com a interface em contato com o usuário. Exatamente como em uma página web, o JavaScript assume aqui as mesmas tarefas que assumiria por lá.

Comportamentos de botões, manipulação de elementos, alteração de textos, essas são as atribuições de responsabilidade do "pato", como alguns chamam o cérebro do HTML.

### Rust

Por questões práticas, o Rust assume a responsabilidade de se comunicar com a API, processando requisições e respostas à API e ao usuário, respectivamente. A praticidade vem exatamente da estrutura compartilhada por ambas as frentes, onde as structs `Author`, `Book`, `Costumer` e `Rental`, criadas em Rust, podem ser reaproveitadas.

### Ambos

Questões como limpeza/tratamento de dados, montagem de HTML, contagem de elementos e renderização de componentes poderiam ser de responsabilidade de qualquer um dos dois lados, são ambos excelentes nesses assuntos, porém, diferentes e exatamente essas diferenças que determinaram qual administra cada questão.

Javascript é extremamente prático, simples de ler, fácil de escrever, direto com suas abordagens, como um painel de um carro. Por sua grande facilidade nessas situações, ele assume as tarefas como tratamento de dados e renderização de componentes.

Rust é extremamente detalhista, intrigante de ler, complexo para escrever, regrado com suas abordagens, como um painél de avião. Ainda que seja mais verboso e rígido, sua velocidade de processamento e segurança são perfeitos para cuidar de tarefas mais pesadas, como contagem de elementos extremamente longos e o processamento + montagem de corpo HTML pesado, carregado.
