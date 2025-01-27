# Operações e Visualização de Vetores

## Visão Geral

Este projeto fornece uma interface web e um servidor backend para realizar operações com vetores e visualizá-los. Ele é implementado utilizando:

- **Backend**: Rust com Actix Web para criar endpoints que tratam operações com vetores.
- **Frontend**: HTML e JavaScript (usando p5.js) para visualização e interação com vetores.

## Funcionalidades

### Backend

O backend expõe os seguintes endpoints:

1. **Soma de Vetores** (`POST /soma`): Soma dois vetores.
2. **Subtração de Vetores** (`POST /subtracao`): Subtrai o segundo vetor do primeiro.
3. **Multiplicação por Escalar** (`POST /redimensionar`): Multiplica um vetor por um escalar.
4. **Produto Escalar** (`POST /produto-escalar`): Calcula o produto escalar de dois vetores.
5. **Produto Vetorial** (`POST /produto-vetorial`): Calcula o produto vetorial de dois vetores (se estiverem em 3D).
6. **Projeção** (`POST /projecao`): Projeta um vetor em outro.
7. **Reflexão** (`POST /reflexao`): Reflete um vetor em outro.

### Frontend

O frontend permite aos usuários:

- Desenhar vetores interativamente.
- Realizar operações como soma e visualizar o vetor resultante.
- Gerar, embaralhar e limpar vetores.
- Visualizar vetores em uma grade 2D com eixos rotulados.

## Começando

### Pré-requisitos

- Rust (com o gerenciador de pacotes `cargo`)
- Navegador web&#x20;

### Configuração

1. Clone o repositório:

   ```bash
   git clone https://github.com/Hactchubas/Vetores
   ```

2. Navegue até o diretório do backend e inicie o servidor:

   ```terminal
   cd .\Vetores\vector-operations\
   cargo run
   ```

   O servidor será iniciado em `http://127.0.0.1:8080`.

3. Abra a visualização do frontend:
   O servidor tenta abrir automaticamente o arquivo `vector-visualization.html` no navegador. Caso não consiga, abra-o manualmente no navegador.

## Endpoints da API

Cada endpoint espera um payload JSON e responde com JSON.

### Exemplo de Payload

```json
{
    "v1": {"dimensions": [1.0, 2.0]},
    "v2": {"dimensions": [3.0, 4.0]},
    "scalar": 2.0
}
```

### Endpoints

| Método | Endpoint            | Descrição                                   |
| ------ | ------------------- | ------------------------------------------- |
| POST   | `/soma`             | Soma dois vetores.                          |
| POST   | `/subtracao`        | Subtrai dois vetores.                       |
| POST   | `/redimensionar`    | Multiplica um vetor por um escalar.         |
| POST   | `/produto-escalar`  | Calcula o produto escalar de dois vetores.  |
| POST   | `/produto-vetorial` | Calcula o produto vetorial de dois vetores. |
| POST   | `/projecao`         | Projeta um vetor em outro.                  |
| POST   | `/reflexao`         | Reflete um vetor em outro.                  |

## Uso

- Use a interface de visualização para desenhar vetores e observar os resultados.
- Interaja com o backend para realizar operações programaticamente utilizando ferramentas como Postman ou `curl`.

## Tecnologias Utilizadas

- **Rust**: Actix Web para APIs do backend
- **p5.js**: Para visualização de vetores 2D
- **HTML/CSS**: Interface do usuário

