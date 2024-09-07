<h1 align="center">BOOKERY - Estrutura Compartilhada</h1>

![GitHub License](https://img.shields.io/github/license/LucasGoncSilva/bookery?labelColor=101010)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/LucasGoncSilva/bookery/unittest.yml?style=flat&labelColor=%23101010)

Atuando como um tipo de hub, a lib `shared` é uma lib interna que cria/concentra todas as estruturas comuns às duas frentes do Bookery, API e Desktop. Sendo mais específico, a lib `shared` administra as structs que os dois pontos do projeto utilizam: `Author`, `Book`, `Costumer` e `Rental` - Autor, Livro, Cliente e Aluguél.

Essa atuação se deve ao fato de que as duas aplicações possuem suas interfaces escrita em Rust, de modo que manter a interface na mesma linguagem das operações presentes nas duas frentes seja extremamente vantajoso por questões de compatibilidade, junto às vantagens que o próprio Rust oferece.

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
│   └── README.md                             # Arquivo principal de leitura
│
├── Cargo.toml                                # Arquivo de dependências do projeto
│
├── package.json                              # Arquivo de gerenciamento de dependências
├── package-lock.json                         # Arquivo de gerenciamento de dependências
│
└── src                                       # Diretório do código-fonte da lib
    │
    ├── lib.rs                                # Arquivo de entrada da lib
    │
    └── structs                               # Diretório de responsabilidade das structs
        ├── mod.rs                            # Arquivo de modularização do diretório
        ├── author.rs                         # Arquivo especialista na struct "Author"
        ├── book.rs                           # Arquivo especialista na struct "Book"
        ├── costumer.rs                       # Arquivo especialista na struct "Costumer"
        └── rental.rs                         # Arquivo especialista na struct "Rental"
```

## Structs

### Author

```rust
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Author {
    pub id: Uuid,
    pub name: PersonName,
    #[serde(with = "super::date_format")]
    pub born: Date,
}

#[derive(Deserialize, Serialize)]
pub struct PayloadAuthor {
    ...
}

#[derive(Deserialize, Serialize)]
pub struct PayloadUpdateAuthor {
    ...
}

impl Author {
    pub fn create(payload_author: PayloadAuthor) -> Result<Self, ConversionError> {
        let name: PersonName = PersonName::try_from(payload_author.name)?;
        let born: Date = payload_author.born;
        let id: Uuid = Uuid::new_v4();

        Ok(Self { id, name, born })
    }

    pub fn parse(author: PayloadUpdateAuthor) -> Result<Self, ConversionError> {
        let name: PersonName = PersonName::try_from(author.name)?;
        let born: Date = author.born;

        Ok(Self {
            id: author.id,
            name,
            born,
        })
    }
}
```

### Book

```rust
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Book {
    pub id: Uuid,
    pub name: BookName,
    pub author_uuid: Uuid,
    pub editor: EditorName,
    #[serde(with = "super::date_format")]
    pub release: Date,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct BookWithAuthor {
    ...
}

#[derive(Deserialize, Serialize)]
pub struct PayloadBook {
    ...
}

#[derive(Deserialize, Serialize)]
pub struct PayloadUpdateBook {
    ...
}

impl Book {
    pub fn create(new_book: PayloadBook) -> Result<Self, ConversionError> {
        let name: BookName = BookName::try_from(new_book.name)?;
        let editor: EditorName = EditorName::try_from(new_book.editor)?;
        let release: Date = new_book.release;
        let author_uuid: Uuid = new_book.author_uuid;
        let id: Uuid = Uuid::new_v4();

        Ok(Self {
            id,
            name,
            author_uuid,
            editor,
            release,
        })
    }

    pub fn parse(book: PayloadUpdateBook) -> Result<Self, ConversionError> {
        let name: BookName = BookName::try_from(book.name)?;
        let editor: EditorName = EditorName::try_from(book.editor)?;
        let release: Date = book.release;
        let author_uuid: Uuid = book.author_uuid;

        Ok(Self {
            id: book.id,
            name,
            author_uuid,
            editor,
            release,
        })
    }
}
```

### Costumer

```rust
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Costumer {
    pub id: Uuid,
    pub name: PersonName,
    pub document: PersonDocument,
    #[serde(with = "super::date_format")]
    pub born: Date,
}

#[derive(Deserialize, Serialize)]
pub struct PayloadCostumer {
    ...
}

#[derive(Deserialize, Serialize)]
pub struct PayloadUpdateCostumer {
    ...
}

impl Costumer {
    pub fn create(new_costumer: PayloadCostumer) -> Result<Self, ConversionError> {
        let name: PersonName = PersonName::try_from(new_costumer.name)?;
        let document: PersonDocument = PersonDocument::try_from(new_costumer.document)?;
        let born: Date = new_costumer.born;
        let id: Uuid = Uuid::new_v4();

        Ok(Self {
            id,
            name,
            document,
            born,
        })
    }

    pub fn parse(costumer: PayloadUpdateCostumer) -> Result<Self, ConversionError> {
        let name: PersonName = PersonName::try_from(costumer.name)?;
        let document: PersonDocument = PersonDocument::try_from(costumer.document)?;
        let born: Date = costumer.born;

        Ok(Self {
            id: costumer.id,
            name,
            document,
            born,
        })
    }
}
```

### Rental

```rust
pub struct Rental {
    pub id: Uuid,
    pub costumer_uuid: Uuid,
    pub book_uuid: Uuid,
    #[serde(with = "super::date_format")]
    pub borrowed_at: Date,
    #[serde(with = "super::date_format")]
    pub due_date: Date,
    #[serde(with = "super::option_date_format")]
    pub returned_at: Option<Date>,
}

#[derive(Serialize, PartialEq, Debug, Deserialize)]
pub struct RentalWithCostumerAndBook {
    ...
}

#[derive(Deserialize, Serialize)]
pub struct PayloadRental {
    ...
}

#[derive(Deserialize, Serialize)]
pub struct PayloadUpdateRental {
    ...
}

impl Rental {
    pub fn create(new_rent: PayloadRental) -> Result<Self, ConversionError> {
        let id: Uuid = Uuid::new_v4();

        Ok(Self {
            id,
            costumer_uuid: new_rent.costumer_uuid,
            book_uuid: new_rent.book_uuid,
            borrowed_at: new_rent.borrowed_at,
            due_date: new_rent.due_date,
            returned_at: None,
        })
    }

    pub fn parse(rent: PayloadUpdateRental) -> Result<Self, ConversionError> {
        Ok(Self {
            id: rent.id,
            costumer_uuid: rent.costumer_uuid,
            book_uuid: rent.book_uuid,
            borrowed_at: rent.borrowed_at,
            due_date: rent.due_date,
            returned_at: rent.returned_at,
        })
    }
}
```
