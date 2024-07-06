### Documentação dos Endpoints - Backend DigitalVault

**Data:** 6 de julho de 2024

---

### Visão Geral

Os endpoints do backend do DigitalVault são expostos via gRPC, proporcionando uma comunicação eficiente e segura entre o frontend e o backend. Esta documentação detalha os serviços gRPC, suas operações e as mensagens utilizadas.

---

### Serviços gRPC

#### 1. UploadService

**Descrição:** Serviço responsável pelo upload de arquivos e criação de novos ativos.

**Métodos:**

- **UploadAsset**

  **Descrição:** Faz o upload de um novo ativo.

  **Request (UploadAssetRequest):**
  - `filename`: string - Nome do arquivo.
  - `file`: bytes - Conteúdo do arquivo.
  - `business_unit_id`: string - Identificador da unidade de negócio.
  - `description`: string - Descrição do ativo.

  **Response (UploadAssetResponse):**
  - `asset_id`: string - Identificador único do ativo criado.

---

#### 2. AssetManagementService

**Descrição:** Serviço responsável pelo gerenciamento de ativos, incluindo listagem, visualização, atualização e exclusão.

**Métodos:**

- **ListAssets**

  **Descrição:** Lista ativos com base em critérios de busca e filtros.

  **Request (ListAssetsRequest):**
  - `business_unit_id`: string - (Opcional) Identificador da unidade de negócio.
  - `filters`: map<string, string> - (Opcional) Filtros adicionais para busca.

  **Response (ListAssetsResponse):**
  - `assets`: repeated Asset - Lista de ativos encontrados.

- **GetAsset**

  **Descrição:** Obtém os detalhes de um ativo específico.

  **Request (GetAssetRequest):**
  - `asset_id`: string - Identificador único do ativo.

  **Response (GetAssetResponse):**
  - `asset`: Asset - Detalhes do ativo.

- **UpdateAsset**

  **Descrição:** Atualiza os metadados de um ativo existente.

  **Request (UpdateAssetRequest):**
  - `asset_id`: string - Identificador único do ativo.
  - `filename`: string - Nome atualizado do arquivo.
  - `description`: string - Descrição atualizada do ativo.

  **Response (UpdateAssetResponse):**
  - `asset`: Asset - Detalhes do ativo atualizado.

- **DeleteAsset**

  **Descrição:** Exclui um ativo do sistema.

  **Request (DeleteAssetRequest):**
  - `asset_id`: string - Identificador único do ativo.

  **Response (DeleteAssetResponse):**
  - `status`: string - Status da operação de exclusão.

---

### Mensagens gRPC

#### UploadAssetRequest

**Descrição:** Mensagem de solicitação para upload de um ativo.

**Campos:**
- `filename`: string - Nome do arquivo.
- `file`: bytes - Conteúdo do arquivo.
- `business_unit_id`: string - Identificador da unidade de negócio.
- `description`: string - Descrição do ativo.

---

#### UploadAssetResponse

**Descrição:** Mensagem de resposta para o upload de um ativo.

**Campos:**
- `asset_id`: string - Identificador único do ativo criado.

---

#### ListAssetsRequest

**Descrição:** Mensagem de solicitação para listar ativos.

**Campos:**
- `business_unit_id`: string - (Opcional) Identificador da unidade de negócio.
- `filters`: map<string, string> - (Opcional) Filtros adicionais para busca.

---

#### ListAssetsResponse

**Descrição:** Mensagem de resposta para a listagem de ativos.

**Campos:**
- `assets`: repeated Asset - Lista de ativos encontrados.

---

#### GetAssetRequest

**Descrição:** Mensagem de solicitação para obter detalhes de um ativo.

**Campos:**
- `asset_id`: string - Identificador único do ativo.

---

#### GetAssetResponse

**Descrição:** Mensagem de resposta para a obtenção de detalhes de um ativo.

**Campos:**
- `asset`: Asset - Detalhes do ativo.

---

#### UpdateAssetRequest

**Descrição:** Mensagem de solicitação para atualizar um ativo.

**Campos:**
- `asset_id`: string - Identificador único do ativo.
- `filename`: string - Nome atualizado do arquivo.
- `description`: string - Descrição atualizada do ativo.

---

#### UpdateAssetResponse

**Descrição:** Mensagem de resposta para a atualização de um ativo.

**Campos:**
- `asset`: Asset - Detalhes do ativo atualizado.

---

#### DeleteAssetRequest

**Descrição:** Mensagem de solicitação para excluir um ativo.

**Campos:**
- `asset_id`: string - Identificador único do ativo.

---

#### DeleteAssetResponse

**Descrição:** Mensagem de resposta para a exclusão de um ativo.

**Campos:**
- `status`: string - Status da operação de exclusão.

---

#### Asset

**Descrição:** Mensagem que representa um ativo digital.

**Campos:**
- `asset_id`: string - Identificador único do ativo.
- `filename`: string - Nome do arquivo.
- `file_path`: string - Caminho do arquivo no MinIO.
- `business_unit_id`: string - Identificador da unidade de negócio.
- `description`: string - Descrição do ativo.
- `created_by`: string - Identificador do usuário que criou o ativo.
- `created_at`: string - Data e hora de criação do ativo.
- `updated_at`: string - Data e hora da última atualização do ativo.

