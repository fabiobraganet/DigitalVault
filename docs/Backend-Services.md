### Documentação da Camada de Serviços - Backend DigitalVault

**Data:** 6 de julho de 2024

---

### Visão Geral da Camada de Serviços

A camada de serviços no backend do DigitalVault é responsável por orquestrar a lógica de negócios, interagir com o domínio e integrar-se com os repositórios e serviços externos. Ela encapsula os casos de uso da aplicação, garantindo que as regras de negócio sejam aplicadas de forma consistente.

---

### Serviços Principais

#### 1. UploadService

**Descrição:** Serviço responsável pelo upload de arquivos e criação de novos ativos.

**Localização:** `src/application/services/upload_service.rs`

**Funcionalidades:**
- Receber solicitações de upload de arquivos.
- Armazenar arquivos no MinIO.
- Salvar metadados dos arquivos no PostgreSQL.
- Publicar eventos de adição de ativos no RabbitMQ.

**Métodos:**
- `upload_asset(request: UploadAssetRequest) -> Result<UploadAssetResponse, ServiceError>`
  - Recebe um arquivo e seus metadados.
  - Armazena o arquivo no MinIO.
  - Salva os metadados no PostgreSQL.
  - Publica um evento `AssetAddedEvent` no RabbitMQ.
  - Retorna uma resposta contendo o ID do ativo criado.

---

#### 2. AssetManagementService

**Descrição:** Serviço responsável pelo gerenciamento de ativos, incluindo listagem, visualização, atualização e exclusão.

**Localização:** `src/application/services/asset_management_service.rs`

**Funcionalidades:**
- Listar ativos com base em critérios de busca e filtros.
- Obter detalhes de um ativo específico.
- Atualizar metadados de ativos.
- Excluir ativos e seus metadados.
- Publicar eventos de atualização e exclusão de ativos no RabbitMQ.

**Métodos:**
- `list_assets(request: ListAssetsRequest) -> Result<ListAssetsResponse, ServiceError>`
  - Busca e retorna uma lista de ativos com base nos critérios de busca e filtros.

- `get_asset(request: GetAssetRequest) -> Result<GetAssetResponse, ServiceError>`
  - Retorna os detalhes de um ativo específico pelo seu ID.

- `update_asset(request: UpdateAssetRequest) -> Result<UpdateAssetResponse, ServiceError>`
  - Atualiza os metadados de um ativo existente.
  - Publica um evento `AssetUpdatedEvent` no RabbitMQ.

- `delete_asset(request: DeleteAssetRequest) -> Result<DeleteAssetResponse, ServiceError>`
  - Remove um ativo e seus metadados do sistema.
  - Publica um evento `AssetDeletedEvent` no RabbitMQ.

---

### Fluxo de Trabalho dos Serviços

#### Upload de Ativos

1. **Recebimento da Solicitação:**
   - O `UploadService` recebe uma solicitação de upload com o arquivo e metadados.
   
2. **Armazenamento do Arquivo:**
   - O arquivo é armazenado no MinIO usando o `MinIO Client`.

3. **Salvamento de Metadados:**
   - Os metadados do arquivo são salvos no PostgreSQL através do `AssetRepository`.

4. **Publicação de Evento:**
   - Um evento `AssetAddedEvent` é publicado no RabbitMQ para notificar outros sistemas sobre o novo ativo.

5. **Retorno da Resposta:**
   - O `UploadService` retorna uma resposta contendo o ID do ativo criado.

---

#### Gerenciamento de Ativos

1. **Listagem de Ativos:**
   - O `AssetManagementService` recebe uma solicitação de listagem.
   - Busca os ativos no PostgreSQL utilizando o `AssetRepository`.
   - Retorna uma lista de ativos.

2. **Visualização de Ativo:**
   - O `AssetManagementService` recebe uma solicitação para visualizar um ativo específico.
   - Busca os detalhes do ativo no PostgreSQL.
   - Retorna os detalhes do ativo.

3. **Atualização de Ativo:**
   - O `AssetManagementService` recebe uma solicitação de atualização de metadados.
   - Atualiza os metadados no PostgreSQL.
   - Publica um evento `AssetUpdatedEvent` no RabbitMQ.
   - Retorna a confirmação da atualização.

4. **Exclusão de Ativo:**
   - O `AssetManagementService` recebe uma solicitação de exclusão de ativo.
   - Remove o arquivo do MinIO.
   - Remove os metadados do PostgreSQL.
   - Publica um evento `AssetDeletedEvent` no RabbitMQ.
   - Retorna a confirmação da exclusão.

---

### Dependências Externas

#### MinIO Client

**Descrição:** Cliente para integração com MinIO para operações de armazenamento de arquivos.

**Localização:** `src/infrastructure/minio/minio_client.rs`

**Métodos:**
- `upload_file(bucket: &str, file: &File) -> Result<(), MinIOError>`
- `delete_file(bucket: &str, file_path: &str) -> Result<(), MinIOError>`
- `get_file(bucket: &str, file_path: &str) -> Result<File, MinIOError>`

---

#### Keycloak Client

**Descrição:** Cliente para integração com Keycloak para autenticação e autorização.

**Localização:** `src/infrastructure/keycloak/keycloak_client.rs`

**Métodos:**
- `validate_token(token: &str) -> Result<UserInfo, KeycloakError>`
- `get_user_info(token: &str) -> Result<UserInfo, KeycloakError>`

---

#### EventPublisher e EventListener

**Descrição:** Componentes para publicação e assinatura de eventos de domínio utilizando RabbitMQ.

**Localização:** `src/infrastructure/rabbitmq/event_publisher.rs` e `src/infrastructure/rabbitmq/event_listener.rs`

**Métodos:**
- `publish(event: &DomainEvent) -> Result<(), RabbitMQError>`
- `subscribe(event_type: &str, handler: fn(&DomainEvent)) -> Result<(), RabbitMQError>`

---

### Interfaces e Repositórios

#### AssetRepository

**Descrição:** Interface e implementação para acessar e manipular dados de ativos no PostgreSQL.

**Localização:** `src/infrastructure/persistence/postgres/asset_repository.rs`

**Métodos:**
- `create_asset(asset: &Asset) -> Result<(), RepositoryError>`
- `update_asset(asset: &Asset) -> Result<(), RepositoryError>`
- `delete_asset(asset_id: &str) -> Result<(), RepositoryError>`
- `find_asset_by_id(asset_id: &str) -> Result<Asset, RepositoryError>`
- `list_assets(filters: &AssetFilters) -> Result<Vec<Asset>, RepositoryError>`

