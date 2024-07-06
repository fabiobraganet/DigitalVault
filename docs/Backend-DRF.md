### Documentação de Requisitos Técnicos - Backend DigitalVault

**Data:** 6 de julho de 2024

---

### Visão Geral

O backend do DigitalVault será desenvolvido em Rust utilizando princípios de Arquitetura Limpa. Ele fornecerá serviços para upload, gerenciamento de ativos digitais, autenticação e autorização, gerenciamento de eventos de domínio, e integração com vários sistemas externos (MinIO, PostgreSQL, RabbitMQ, Keycloak). A comunicação entre o frontend e o backend será realizada via gRPC.

---

### Requisitos Funcionais

#### 1. Autenticação e Autorização

**Descrição:** O backend deve integrar-se com Keycloak para autenticação e autorização de usuários.

**Funcionalidades:**
- Validar tokens JWT fornecidos pelo frontend.
- Obter informações de usuários a partir do Keycloak.
- Implementar middleware para proteger endpoints com base em papéis e permissões.

**Endpoints:**
- N/A (middleware de autenticação será aplicado em todos os endpoints necessários).

---

#### 2. Upload de Ativos

**Descrição:** O backend deve permitir o upload de arquivos, armazená-los no MinIO e salvar os metadados no PostgreSQL.

**Funcionalidades:**
- Receber arquivos e metadados via gRPC.
- Armazenar arquivos no MinIO.
- Salvar metadados (nome do arquivo, descrição, unidade de negócio, etc.) no PostgreSQL.
- Publicar eventos de adição de ativos no RabbitMQ.

**Endpoints gRPC:**
- `UploadService.UploadAsset(UploadAssetRequest) -> UploadAssetResponse`

---

#### 3. Gerenciamento de Ativos

**Descrição:** O backend deve permitir a visualização, edição e exclusão de ativos.

**Funcionalidades:**
- Listar ativos com base em critérios de busca e filtros.
- Obter detalhes de um ativo específico.
- Atualizar metadados de ativos.
- Excluir ativos e seus metadados.
- Publicar eventos de atualização e exclusão de ativos no RabbitMQ.

**Endpoints gRPC:**
- `AssetManagementService.ListAssets(ListAssetsRequest) -> ListAssetsResponse`
- `AssetManagementService.GetAsset(GetAssetRequest) -> GetAssetResponse`
- `AssetManagementService.UpdateAsset(UpdateAssetRequest) -> UpdateAssetResponse`
- `AssetManagementService.DeleteAsset(DeleteAssetRequest) -> DeleteAssetResponse`

---

#### 4. Armazenamento Seguro de Arquivos

**Descrição:** O backend deve integrar-se com MinIO para armazenamento seguro de arquivos.

**Funcionalidades:**
- Conectar-se ao MinIO utilizando a API S3.
- Armazenar, recuperar e deletar arquivos no MinIO.
- Gerenciar buckets e objetos no MinIO.

**Endpoints gRPC:**
- Incluídos nos serviços de upload e gerenciamento de ativos.

---

#### 5. Eventos de Domínio

**Descrição:** O backend deve gerenciar eventos de domínio utilizando RabbitMQ.

**Funcionalidades:**
- Publicar eventos de adição, atualização e exclusão de ativos no RabbitMQ.
- Processar eventos de domínio e realizar ações correspondentes.

**Endpoints gRPC:**
- N/A (gerenciamento de eventos é interno).

---

### Requisitos Não Funcionais

#### 1. Segurança

**Descrição:** O backend deve garantir a segurança dos dados e a comunicação entre sistemas.

**Medidas:**
- Utilização de TLS para comunicação gRPC.
- Validação de tokens JWT.
- Proteção de endpoints com base em papéis e permissões.

---

#### 2. Escalabilidade

**Descrição:** O backend deve ser capaz de escalar horizontalmente para suportar um grande volume de usuários e arquivos.

**Medidas:**
- Uso de RabbitMQ para processamento assíncrono de eventos.
- Armazenamento de objetos em MinIO, que pode ser escalado horizontalmente.
- PostgreSQL configurado para replicação e escalabilidade.

---

#### 3. Performance

**Descrição:** O backend deve responder a solicitações de forma rápida e eficiente.

**Medidas:**
- Utilização de gRPC para comunicação eficiente.
- Otimização de consultas ao PostgreSQL.
- Uso de caches, quando aplicável.

---

#### 4. Manutenibilidade

**Descrição:** O código do backend deve ser fácil de entender, manter e evoluir.

**Medidas:**
- Aplicação de princípios de Arquitetura Limpa e SOLID.
- Documentação clara e concisa.
- Testes automatizados para garantir a qualidade do código.

---

### Estrutura do Projeto

**Diretórios:**
- `src/domain/`: Entidades e lógica de domínio.
- `src/application/`: Serviços e casos de uso.
- `src/infrastructure/`: Integrações com sistemas externos.
- `src/presentation/`: APIs e controladores.
- `src/main.rs`: Ponto de entrada da aplicação.

---

### Tecnologias e Ferramentas

- **Linguagem:** Rust
- **Framework Web:** Actix-web
- **ORM:** Diesel
- **Armazenamento de Arquivos:** MinIO
- **Banco de Dados:** PostgreSQL
- **Mensageria:** RabbitMQ
- **Autenticação:** Keycloak
- **Comunicação:** gRPC

---

### Detalhamento dos Componentes

#### 1. Keycloak Client

**Descrição:** Cliente para integração com Keycloak.

**Funcionalidades:**
- Validação de tokens JWT.
- Obtenção de informações do usuário.

**Localização:** `src/infrastructure/keycloak/`

---

#### 2. UploadService

**Descrição:** Serviço para upload de arquivos.

**Funcionalidades:**
- Receber e processar solicitações de upload.
- Armazenar arquivos no MinIO.
- Salvar metadados no PostgreSQL.
- Publicar eventos de adição de ativos no RabbitMQ.

**Localização:** `src/application/services/`

---

#### 3. AssetManagementService

**Descrição:** Serviço para gerenciamento de ativos.

**Funcionalidades:**
- Listar, visualizar, editar e excluir ativos.
- Publicar eventos de atualização e exclusão de ativos no RabbitMQ.

**Localização:** `src/application/services/`

---

#### 4. MinIO Client

**Descrição:** Cliente para integração com MinIO.

**Funcionalidades:**
- Conectar-se ao MinIO.
- Armazenar, recuperar e deletar arquivos.

**Localização:** `src/infrastructure/minio/`

---

#### 5. Repositórios

**Descrição:** Interfaces e implementações para acessar e manipular dados no PostgreSQL.

**Funcionalidades:**
- Operações CRUD para metadados de ativos.
- Consultas otimizadas.

**Localização:** `src/infrastructure/persistence/postgres/`

---

#### 6. EventPublisher e EventListener

**Descrição:** Publicador e assinante de eventos de domínio utilizando RabbitMQ.

**Funcionalidades:**
- Publicar eventos de adição, atualização e exclusão de ativos.
- Processar eventos de domínio recebidos.

**Localização:** `src/infrastructure/rabbitmq/`

---

#### 7. gRPC Server

**Descrição:** Servidor gRPC para atender solicitações do frontend.

**Funcionalidades:**
- Implementar serviços definidos nos arquivos `.proto`.

**Localização:** `src/presentation/grpc_server.rs`

