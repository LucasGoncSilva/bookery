<h1 align="center">
  <img src="./logo.svg" height="300" width="300" alt="Logo BOOKERY" /><br>
  BOOKERY
</h1>

![GitHub License](https://img.shields.io/github/license/LucasGoncSilva/bookery?labelColor=101010)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/LucasGoncSilva/bookery/unittest.yml?style=flat&labelColor=%23101010)

Criado em Axum e Tauti, Frameworks Rust para API e Desktop, respectivamente, Bookery é um mini-sistema desktop para bibliotecas realizarem o gerenciamento de seus livros e empréstimos.
<br><br>
O Bookery permite o cadastro e a busca eficiente de autores, livros, clientes e aluguéis, oferecendo uma experiência completa de gerenciamento com filtros avançados, opções de edição e uma interface intuitiva. Cada funcionalidade foi projetada para garantir que bibliotecas possam administrar suas coleções e transações de maneira ágil e precisa.
<br><br>
Desenvolvido com um foco rigoroso em qualidade, o sistema é avaliado por mais de 140 testes automatizados, garantindo robustez e confiabilidade. Aproveitando a eficiência do Rust, o Bookery supera soluções como o Electron em termos de processamento e uso de memória, enquanto a API em Axum proporciona uma performance comparável a C/C++, oferecendo um desempenho superior com simplicidade e eficácia.

<br>

## Stack

![Tauri logo](https://img.shields.io/badge/Tauri-0f0f0f?style=for-the-badge&logo=Tauri&logoColor=f7bb2f)

![HTML logo](https://img.shields.io/badge/HTML5-E34F26?style=for-the-badge&logo=html5&logoColor=white)
![CSS logo](https://img.shields.io/badge/CSS3-1572B6?style=for-the-badge&logo=css3&logoColor=white)
![Sass logo](https://img.shields.io/badge/Sass-CC6699?style=for-the-badge&logo=sass&logoColor=white)
![JavaScript logo](https://img.shields.io/badge/JavaScript-323330?style=for-the-badge&logo=javascript&logoColor=F7DF1E)

![Rust logo for Axum](https://img.shields.io/badge/Axum-ef4900?style=for-the-badge&logo=rust&logoColor=white)

![PostgreSQL logo](https://img.shields.io/badge/PostgreSQL-316192?style=for-the-badge&logo=postgresql&logoColor=white)

![Docker logo](https://img.shields.io/badge/Docker-2CA5E0?style=for-the-badge&logo=docker&logoColor=white)

<br>

## Arquitetura

A arquitetura pode ser detalhada de forma geral em duas frentes: Desktop e API. Tratando a aplicação Desktop como cliente desta solução, será detalhada uma visão comportamental em escala "macro" seguindo o fluxo de dados sem focar no "micro", como cada ação de cada função. Acompanhe abaixo o fluxo geral de informações:

![Arquitetura Geral](./arch.svg)

<br>

## Observação

Os detalhes de cada face do sistema estão descritos dentro de seus próprios módulos. Considere acessar os endereços abaixo para visualizar os detalhes da arquitetura de cada módulo:

- [Arquitetura Desktop](https://github.com/LucasGoncSilva/bookery/tree/main/BOOKERY/desktop)
- [Arquitetura API](https://github.com/LucasGoncSilva/bookery/tree/main/BOOKERY/api)
- [Arquitetura Compartilhada](https://github.com/LucasGoncSilva/bookery/tree/main/BOOKERY/shared)

<br>

## Licença

This project is under [MPLv2 - Mozilla Public License Version 2.0](https://choosealicense.com/licenses/mpl-2.0/). Permissions of this weak copyleft license are conditioned on making available source code of licensed files and modifications of those files under the same license (or in certain cases, one of the GNU licenses). Copyright and license notices must be preserved. Contributors provide an express grant of patent rights. However, a larger work using the licensed work may be distributed under different terms and without source code for files added in the larger work.
