### Documentação do Domínio - Backend DigitalVault

**Data:** 6 de julho de 2024

---

### Visão Geral do Domínio

O domínio do backend do DigitalVault define as entidades e a lógica central que governam as regras de negócios e operações fundamentais do sistema de gerenciamento de ativos digitais. A aplicação de princípios de Arquitetura Limpa garante que o domínio seja independente de frameworks, bibliotecas e detalhes de infraestrutura, facilitando a manutenção e evolução do sistema.

---

### Entidades Principais

#### 1. Usuário (User)

**Descrição:** Representa um usuário do sistema, autenticado via Keycloak.

**Atributos:**
- `user_id`: Identificador único do usuário.
- `username`: Nome de usuário.
- `email`: Endereço de e-mail.
- `roles`: Lista de papéis atribuídos ao usuário (e.g., admin, editor, visualizador).

**Regras de Negócio:**
- Um usuário pode ter múltiplos papéis.
- As permissões de acesso são determinadas pelos papéis atribuídos ao usuário.

**Localização:** `src/domain/entities/user.rs`

---

#### 2. Ativo (Asset)

**Descrição:** Representa um ativo digital que pode ser um arquivo de qualquer tipo (imagem, vídeo, documento, etc.).

**Atributos:**
- `asset_id`: Identificador único do ativo.
- `filename`: Nome do arquivo.
- `file_path`: Caminho do arquivo no MinIO.
- `business_unit_id`: Identificador da unidade de negócio à qual o ativo pertence.
- `description`: Descrição do ativo.
- `created_by`: Identificador do usuário que criou o ativo.
- `created_at`: Data e hora de criação do ativo.
- `updated_at`: Data e hora da última atualização do ativo.

**Regras de Negócio:**
- Cada ativo deve estar associado a uma unidade de negócio.
- Apenas usuários com permissões adequadas podem editar ou excluir um ativo.

**Localização:** `src/domain/entities/asset.rs`

---

#### 3. Unidade de Negócio (BusinessUnit)

**Descrição:** Representa uma unidade de negócio dentro da organização.

**Atributos:**
- `business_unit_id`: Identificador único da unidade de negócio.
- `name`: Nome da unidade de negócio.
- `description`: Descrição da unidade de negócio.

**Regras de Negócio:**
- Cada ativo deve estar associado a uma unidade de negócio.
- Unidades de negócio podem ser usadas para segmentar e organizar ativos.

**Localização:** `src/domain/entities/business_unit.rs`

---

### Eventos de Domínio

#### 1. Evento de Adição de Ativo (AssetAddedEvent)

**Descrição:** Evento disparado quando um novo ativo é adicionado ao sistema.

**Atributos:**
- `asset_id`: Identificador único do ativo.
- `business_unit_id`: Identificador da unidade de negócio à qual o ativo pertence.
- `filename`: Nome do arquivo.
- `created_by`: Identificador do usuário que criou o ativo.
- `created_at`: Data e hora de criação do ativo.

**Localização:** `src/domain/events/asset_added_event.rs`

---

#### 2. Evento de Atualização de Ativo (AssetUpdatedEvent)

**Descrição:** Evento disparado quando um ativo é atualizado.

**Atributos:**
- `asset_id`: Identificador único do ativo.
- `business_unit_id`: Identificador da unidade de negócio à qual o ativo pertence.
- `filename`: Nome do arquivo.
- `updated_by`: Identificador do usuário que atualizou o ativo.
- `updated_at`: Data e hora da última atualização do ativo.

**Localização:** `src/domain/events/asset_updated_event.rs`

---

#### 3. Evento de Exclusão de Ativo (AssetDeletedEvent)

**Descrição:** Evento disparado quando um ativo é excluído.

**Atributos:**
- `asset_id`: Identificador único do ativo.
- `deleted_by`: Identificador do usuário que excluiu o ativo.
- `deleted_at`: Data e hora da exclusão do ativo.

**Localização:** `src/domain/events/asset_deleted_event.rs`

---

### Casos de Uso

#### 1. Adicionar Ativo

**Descrição:** Caso de uso para adicionar um novo ativo ao sistema.

**Fluxo:**
1. O usuário envia uma solicitação de upload com o arquivo e metadados.
2. O serviço de upload armazena o arquivo no MinIO.
3. Os metadados do ativo são salvos no PostgreSQL.
4. Um evento `AssetAddedEvent` é publicado no RabbitMQ.

**Localização:** `src/application/services/upload_service.rs`

---

#### 2. Atualizar Ativo

**Descrição:** Caso de uso para atualizar os metadados de um ativo existente.

**Fluxo:**
1. O usuário envia uma solicitação de atualização com os novos metadados.
2. Os metadados do ativo são atualizados no PostgreSQL.
3. Um evento `AssetUpdatedEvent` é publicado no RabbitMQ.

**Localização:** `src/application/services/asset_management_service.rs`

---

#### 3. Excluir Ativo

**Descrição:** Caso de uso para excluir um ativo do sistema.

**Fluxo:**
1. O usuário envia uma solicitação de exclusão.
2. O arquivo é removido do MinIO.
3. Os metadados do ativo são removidos do PostgreSQL.
4. Um evento `AssetDeletedEvent` é publicado no RabbitMQ.

**Localização:** `src/application/services/asset_management_service.rs`

---

### Repositórios

**Descrição:** Interfaces e implementações para acessar e manipular dados no PostgreSQL.

**Classes e Localização:**
- `AssetRepository`: `src/infrastructure/persistence/postgres/asset_repository.rs`
- `BusinessUnitRepository`: `src/infrastructure/persistence/postgres/business_unit_repository.rs`
- `UserRepository`: `src/infrastructure/persistence/postgres/user_repository.rs`

---

### Serviços de Aplicação

#### UploadService

**Descrição:** Serviço responsável pelo upload de arquivos e criação de novos ativos.

**Localização:** `src/application/services/upload_service.rs`

---

#### AssetManagementService

**Descrição:** Serviço responsável pelo gerenciamento de ativos, incluindo listagem, visualização, atualização e exclusão.

**Localização:** `src/application/services/asset_management_service.rs`

---

### Integração com Sistemas Externos

#### MinIO Client

**Descrição:** Cliente para integração com MinIO para operações de armazenamento de arquivos.

**Localização:** `src/infrastructure/minio/minio_client.rs`

---

#### Keycloak Client

**Descrição:** Cliente para integração com Keycloak para autenticação e autorização.

**Localização:** `src/infrastructure/keycloak/keycloak_client.rs`

---

#### EventPublisher e EventListener

**Descrição:** Componentes para publicação e assinatura de eventos de domínio utilizando RabbitMQ.

**Localização:** `src/infrastructure/rabbitmq/event_publisher.rs` e `src/infrastructure/rabbitmq/event_listener.rs`

