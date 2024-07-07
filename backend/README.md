# DigitalVault
> Plataforma de gerenciamento de ativos digitais

---

### README - Backend DigitalVault

**Data:** 6 de julho de 2024

---

# Backend

DigitalVault é uma plataforma robusta e segura para o gerenciamento de ativos digitais. Este repositório contém o código-fonte do backend, desenvolvido em Rust, utilizando MinIO para armazenamento de arquivos, PostgreSQL para banco de dados, RabbitMQ para mensageria e gRPC para comunicação eficiente com o frontend.

## Visão Geral

O backend do DigitalVault é responsável por gerenciar a lógica de negócios, armazenamento seguro de arquivos, autenticação e autorização, bem como a comunicação com outros serviços. A arquitetura segue princípios de Arquitetura Limpa e SOLID para garantir manutenibilidade e escalabilidade.

## Tecnologias Utilizadas

- **Linguagem:** Rust
- **Framework Web:** Actix-web
- **ORM:** Diesel
- **Armazenamento de Arquivos:** MinIO
- **Banco de Dados:** PostgreSQL
- **Mensageria:** RabbitMQ
- **Autenticação:** Keycloak
- **Comunicação:** gRPC
- **Infraestrutura:** Docker, Docker Compose

## Estrutura do Projeto

```plaintext
digitalvault-backend/
├── Cargo.lock
├── Cargo.toml
├── README.md
├── diesel.toml
├── .env
├── .gitignore
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── domain/
│   │   ├── entities/
│   │   │   ├── asset.rs
│   │   │   ├── business_unit.rs
│   │   │   └── user.rs
│   │   ├── events/
│   │   │   ├── asset_added_event.rs
│   │   │   ├── asset_updated_event.rs
│   │   │   └── asset_deleted_event.rs
│   │   └── mod.rs
│   ├── application/
│   │   ├── services/
│   │   │   ├── mod.rs
│   │   │   ├── upload_service.rs
│   │   │   └── asset_management_service.rs
│   │   └── mod.rs
│   ├── infrastructure/
│   │   ├── grpc/
│   │   │   ├── mod.rs
│   │   │   ├── upload_service_impl.rs
│   │   │   └── asset_management_service_impl.rs
│   │   ├── persistence/
│   │   │   ├── postgres/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── asset_repository.rs
│   │   │   │   ├── business_unit_repository.rs
│   │   │   │   └── user_repository.rs
│   │   └── mod.rs
│   │   ├── minio/
│   │   │   ├── mod.rs
│   │   │   └── minio_client.rs
│   │   ├── keycloak/
│   │   │   ├── mod.rs
│   │   │   └── keycloak_client.rs
│   │   ├── rabbitmq/
│   │   │   ├── mod.rs
│   │   │   ├── event_publisher.rs
│   │   │   └── event_listener.rs
│   ├── presentation/
│   │   ├── grpc_server.rs
│   │   └── mod.rs
├── proto/
│   ├── upload.proto
│   └── asset_management.proto
├── migrations/
│   ├── 2024-07-06-000001_create_assets_table.sql
│   ├── 2024-07-06-000002_create_business_units_table.sql
│   └── 2024-07-06-000003_create_users_table.sql
```

## Configuração do Ambiente

### Pré-requisitos

- [Rust](https://www.rust-lang.org/)
- [Docker](https://www.docker.com/)
- [Docker Compose](https://docs.docker.com/compose/)
- [PostgreSQL](https://www.postgresql.org/)
- [MinIO](https://min.io/)
- [RabbitMQ](https://www.rabbitmq.com/)
- [Keycloak](https://www.keycloak.org/)

### Configuração

1. Configure as variáveis de ambiente no arquivo `.env`:

```plaintext
DATABASE_URL=postgres://username:password@localhost/digitalvault
MINIO_ENDPOINT=http://localhost:9000
MINIO_ACCESS_KEY=minio
MINIO_SECRET_KEY=minio123
RABBITMQ_URL=amqp://localhost:5672
KEYCLOAK_URL=http://localhost:8080/auth
KEYCLOAK_REALM=digitalvault
KEYCLOAK_CLIENT_ID=digitalvault-backend
KEYCLOAK_CLIENT_SECRET=secret
```

2. Configure o Docker Compose para iniciar os serviços necessários:

```bash
docker-compose up -d
```

3. Execute as migrações do banco de dados:

```bash
cargo install diesel_cli --no-default-features --features postgres
diesel migration run
```

4. Inicie a aplicação:

```bash
cargo run
```

## Endpoints gRPC

### UploadService

**UploadAsset**

- **Descrição:** Faz o upload de um novo ativo.
- **Request:** `UploadAssetRequest`
  - `filename`: string
  - `file`: bytes
  - `business_unit_id`: string
  - `description`: string
- **Response:** `UploadAssetResponse`
  - `asset_id`: string

### AssetManagementService

**ListAssets**

- **Descrição:** Lista ativos com base em critérios de busca e filtros.
- **Request:** `ListAssetsRequest`
  - `business_unit_id`: string (opcional)
  - `filters`: map<string, string> (opcional)
- **Response:** `ListAssetsResponse`
  - `assets`: repeated Asset

**GetAsset**

- **Descrição:** Obtém os detalhes de um ativo específico.
- **Request:** `GetAssetRequest`
  - `asset_id`: string
- **Response:** `GetAssetResponse`
  - `asset`: Asset

**UpdateAsset**

- **Descrição:** Atualiza os metadados de um ativo existente.
- **Request:** `UpdateAssetRequest`
  - `asset_id`: string
  - `filename`: string
  - `description`: string
- **Response:** `UpdateAssetResponse`
  - `asset`: Asset

**DeleteAsset**

- **Descrição:** Exclui um ativo do sistema.
- **Request:** `DeleteAssetRequest`
  - `asset_id`: string
- **Response:** `DeleteAssetResponse`
  - `status`: string



