# Twitter API com Rust

## Getting Started
### Pre requisitos

Rust
Diesel


## Instalação

Para que essa API tenha o seu funcionamento correto é necessário que siga os passos descritos abaixo:

### Criação do Database

Para a criação do database é preciso rodar o seguinte comando:

```bash
diesel setup
```
Após esse passo, as migration devem ser rodadas com o seguinte comando:

```bash
diesel migration run
```

Nota: A variável DATABASE_URL contida no arquivo .end deve fazer o apontamento correto para o banco postgreSQL.

