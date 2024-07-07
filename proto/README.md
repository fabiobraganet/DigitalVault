# DigitalVault
> Plataforma de gerenciamento de ativos digitais

---

### README - DigitalVault Proto

**Data:** 6 de julho de 2024

---

# DigitalVault - Protobuf Definitions

Este repositório contém as definições de mensagens e serviços gRPC usados no projeto DigitalVault. As definições são escritas em Protocol Buffers (Protobuf), uma linguagem neutra e independente de plataforma usada para serializar dados estruturados. O gRPC utiliza essas definições para gerar código que facilita a comunicação eficiente entre o frontend e o backend.

## Visão Geral

As definições Protobuf são essenciais para a comunicação entre o frontend e o backend do DigitalVault. Este repositório contém as mensagens e serviços necessários para o gerenciamento de ativos digitais, incluindo upload, listagem, visualização, atualização e exclusão de ativos.

## Estrutura do Projeto

```plaintext
digitalvault-proto/
├── upload.proto
└── asset_management.proto
```

## Serviços e Mensagens

### UploadService

**Descrição:** Serviço responsável pelo upload de arquivos e criação de novos ativos.

| Método        | Descrição                      | Request                     | Response                     |
|---------------|--------------------------------|-----------------------------|------------------------------|
| UploadAsset   | Faz o upload de um novo ativo. | UploadAssetRequest          | UploadAssetResponse          |

#### UploadAssetRequest

| Campo             | Tipo     | Descrição                     |
|-------------------|----------|-------------------------------|
| filename          | string   | Nome do arquivo.              |
| file              | bytes    | Conteúdo do arquivo.          |
| business_unit_id  | string   | Identificador da unidade de negócio. |
| description       | string   | Descrição do ativo.           |

#### UploadAssetResponse

| Campo             | Tipo     | Descrição                     |
|-------------------|----------|-------------------------------|
| asset_id          | string   | Identificador único do ativo criado. |

---

### AssetManagementService

**Descrição:** Serviço responsável pelo gerenciamento de ativos, incluindo listagem, visualização, atualização e exclusão.

| Método            | Descrição                          | Request                       | Response                        |
|-------------------|------------------------------------|-------------------------------|---------------------------------|
| ListAssets        | Lista ativos com base em critérios de busca e filtros. | ListAssetsRequest            | ListAssetsResponse              |
| GetAsset          | Obtém os detalhes de um ativo específico. | GetAssetRequest              | GetAssetResponse                |
| UpdateAsset       | Atualiza os metadados de um ativo existente. | UpdateAssetRequest           | UpdateAssetResponse             |
| DeleteAsset       | Exclui um ativo do sistema.        | DeleteAssetRequest            | DeleteAssetResponse             |

#### ListAssetsRequest

| Campo             | Tipo     | Descrição                     |
|-------------------|----------|-------------------------------|
| business_unit_id  | string   | (Opcional) Identificador da unidade de negócio. |
| filters           | map<string, string> | (Opcional) Filtros adicionais para busca. |

#### ListAssetsResponse

| Campo             | Tipo     | Descrição                     |
|-------------------|----------|-------------------------------|
| assets            | repeated Asset | Lista de ativos encontrados. |

#### GetAssetRequest

| Campo             | Tipo     | Descrição                     |
|-------------------|----------|-------------------------------|
| asset_id          | string   | Identificador único do ativo. |

#### GetAssetResponse

| Campo             | Tipo     | Descrição                     |
|-------------------|----------|-------------------------------|
| asset             | Asset    | Detalhes do ativo.            |

#### UpdateAssetRequest

| Campo             | Tipo     | Descrição                     |
|-------------------|----------|-------------------------------|
| asset_id          | string   | Identificador único do ativo. |
| filename          | string   | Nome atualizado do arquivo.   |
| description       | string   | Descrição atualizada do ativo.|

#### UpdateAssetResponse

| Campo             | Tipo     | Descrição                     |
|-------------------|----------|-------------------------------|
| asset             | Asset    | Detalhes do ativo atualizado. |

#### DeleteAssetRequest

| Campo             | Tipo     | Descrição                     |
|-------------------|----------|-------------------------------|
| asset_id          | string   | Identificador único do ativo. |

#### DeleteAssetResponse

| Campo             | Tipo     | Descrição                     |
|-------------------|----------|-------------------------------|
| status            | string   | Status da operação de exclusão.|

#### Asset

| Campo             | Tipo     | Descrição                     |
|-------------------|----------|-------------------------------|
| asset_id          | string   | Identificador único do ativo. |
| filename          | string   | Nome do arquivo.              |
| file_path         | string   | Caminho do arquivo no MinIO.  |
| business_unit_id  | string   | Identificador da unidade de negócio. |
| description       | string   | Descrição do ativo.           |
| created_by        | string   | Identificador do usuário que criou o ativo. |
| created_at        | string   | Data e hora de criação do ativo. |
| updated_at        | string   | Data e hora da última atualização do ativo. |

---

## Configuração do Ambiente

### Pré-requisitos

- [Protocol Buffers Compiler (protoc)](https://grpc.io/docs/protoc-installation/)
- Plugins para gRPC e Protobuf nas linguagens necessárias (por exemplo, protoc-gen-go para Go, grpc_tools_node_protoc_plugin para Node.js)

### Compilação das Definições

Compile as definições Protobuf:

```bash
protoc --proto_path=. --<language>_out=. --grpc_<language>_out=. upload.proto asset_management.proto
```

Substitua `<language>` pela linguagem de programação desejada (por exemplo, `go`, `js`, `java`).

