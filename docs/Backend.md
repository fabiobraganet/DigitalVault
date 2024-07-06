### Estrutura do Projeto Backend - DigitalVault

**Data:** 6 de julho de 2024

---

### Visão Geral

A estrutura do projeto backend do DigitalVault segue os princípios de Arquitetura Limpa e organiza o código de maneira modular e fácil de manter. Abaixo está a árvore completa do sistema de arquivos do projeto.

---

### Árvore Completa do Sistema de Arquivos

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

---

### Descrição dos Diretórios e Arquivos

#### 1. Raiz do Projeto

- **Cargo.lock**: Lockfile para gerenciar versões exatas das dependências Rust.
- **Cargo.toml**: Arquivo de configuração do Cargo, especificando dependências e metadados do projeto.
- **README.md**: Documentação do projeto.
- **diesel.toml**: Configuração do Diesel ORM.
- **.env**: Arquivo de configuração de variáveis de ambiente.
- **.gitignore**: Arquivo especificando quais arquivos e diretórios devem ser ignorados pelo Git.

#### 2. src/

- **main.rs**: Ponto de entrada da aplicação.
- **lib.rs**: Arquivo de configuração de biblioteca (se necessário).

##### 2.1. domain/

- **entities/**: Diretório contendo definições das entidades de domínio.
  - **asset.rs**: Definição da entidade `Asset`.
  - **business_unit.rs**: Definição da entidade `BusinessUnit`.
  - **user.rs**: Definição da entidade `User`.

- **events/**: Diretório contendo definições de eventos de domínio.
  - **asset_added_event.rs**: Definição do evento `AssetAddedEvent`.
  - **asset_updated_event.rs**: Definição do evento `AssetUpdatedEvent`.
  - **asset_deleted_event.rs**: Definição do evento `AssetDeletedEvent`.

- **mod.rs**: Arquivo principal para a configuração do módulo `domain`.

##### 2.2. application/

- **services/**: Diretório contendo serviços de aplicação.
  - **mod.rs**: Arquivo principal para a configuração do módulo `services`.
  - **upload_service.rs**: Implementação do serviço `UploadService`.
  - **asset_management_service.rs**: Implementação do serviço `AssetManagementService`.

- **mod.rs**: Arquivo principal para a configuração do módulo `application`.

##### 2.3. infrastructure/

- **grpc/**: Diretório contendo implementações gRPC.
  - **mod.rs**: Arquivo principal para a configuração do módulo `grpc`.
  - **upload_service_impl.rs**: Implementação gRPC do serviço `UploadService`.
  - **asset_management_service_impl.rs**: Implementação gRPC do serviço `AssetManagementService`.

- **persistence/**: Diretório contendo repositórios e acesso a dados.
  - **postgres/**: Diretório específico para implementações do PostgreSQL.
    - **mod.rs**: Arquivo principal para a configuração do módulo `postgres`.
    - **asset_repository.rs**: Implementação do repositório `AssetRepository`.
    - **business_unit_repository.rs**: Implementação do repositório `BusinessUnitRepository`.
    - **user_repository.rs**: Implementação do repositório `UserRepository`.

- **minio/**: Diretório contendo integração com MinIO.
  - **mod.rs**: Arquivo principal para a configuração do módulo `minio`.
  - **minio_client.rs**: Implementação do cliente MinIO.

- **keycloak/**: Diretório contendo integração com Keycloak.
  - **mod.rs**: Arquivo principal para a configuração do módulo `keycloak`.
  - **keycloak_client.rs**: Implementação do cliente Keycloak.

- **rabbitmq/**: Diretório contendo integração com RabbitMQ.
  - **mod.rs**: Arquivo principal para a configuração do módulo `rabbitmq`.
  - **event_publisher.rs**: Implementação do publicador de eventos.
  - **event_listener.rs**: Implementação do assinante de eventos.

- **mod.rs**: Arquivo principal para a configuração do módulo `infrastructure`.

##### 2.4. presentation/

- **grpc_server.rs**: Implementação do servidor gRPC.
- **mod.rs**: Arquivo principal para a configuração do módulo `presentation`.

#### 3. proto/

- **upload.proto**: Definição do serviço e mensagens gRPC para upload.
- **asset_management.proto**: Definição do serviço e mensagens gRPC para gerenciamento de ativos.

#### 4. migrations/

- **2024-07-06-000001_create_assets_table.sql**: Migração para criação da tabela `assets`.
- **2024-07-06-000002_create_business_units_table.sql**: Migração para criação da tabela `business_units`.
- **2024-07-06-000003_create_users_table.sql**: Migração para criação da tabela `users`.
