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

---

## Definições e boas práticas

### Visão Geral

Este documento fornece uma visão detalhada de como as definições Protobuf são utilizadas para facilitar a comunicação entre o frontend (ReactJS + TypeScript) e o backend (Rust) no projeto DigitalVault. Também incluímos um mapa de comunicação considerando a integração com Keycloak e um manual de boas práticas para implementação e uso das definições Protobuf.

---

### Mapa de Comunicação

**Componentes Envolvidos:**
1. **Frontend:** Desenvolvido em ReactJS + TypeScript, comunicando-se com o backend via gRPC.
2. **Backend:** Desenvolvido em Rust, implementando serviços gRPC.
3. **Keycloak:** Gerenciamento de autenticação e autorização.
4. **Protobuf:** Definições para gRPC.

**Fluxo de Comunicação:**

1. **Autenticação:**
   - O usuário faz login no frontend.
   - O frontend redireciona para o Keycloak para autenticação.
   - Keycloak valida as credenciais e redireciona de volta ao frontend com um token JWT.

2. **Requisição de Serviço:**
   - O frontend utiliza o token JWT para fazer uma requisição gRPC ao backend.
   - O backend valida o token JWT com Keycloak.
   - Se o token for válido, o backend processa a requisição e retorna a resposta.

**Diagrama de Sequência:**

```plaintext
Frontend         Keycloak         Backend
    |                |                |
    |-- Login Req -->|                |
    |                |-- Auth Req --> |
    |                |<-- Auth Res -- |
    |<-- Token ----- |                |
    |-- gRPC Req -------------------->|
    |                                 |
    |<--------- gRPC Res -------------|
```

---

### Boas Práticas para Implementação

#### 1. Definições Protobuf

- **Manter Definições Claras e Concisas:**
  Defina mensagens e serviços de forma clara e concisa, com descrições de campos bem documentadas.

- **Versão do Protocolo:**
  Utilize uma versão consistente do Protocolo Buffers para evitar problemas de compatibilidade.

- **Modularidade:**
  Mantenha as definições modulares e organizadas por contexto funcional.

#### 2. Implementação no Frontend (ReactJS + TypeScript)

**Instalação de Dependências:**

```bash
npm install @grpc/grpc-js @grpc/proto-loader google-protobuf
```

**Exemplo de Implementação:**

```typescript
import * as grpc from '@grpc/grpc-js';
import * as protoLoader from '@grpc/proto-loader';

// Carregar definições Protobuf
const packageDefinition = protoLoader.loadSync('path/to/upload.proto', {});
const proto = grpc.loadPackageDefinition(packageDefinition).upload;

// Configurar cliente gRPC
const client = new proto.UploadService('localhost:50051', grpc.credentials.createInsecure());

// Função para fazer upload de ativo
function uploadAsset(filename: string, file: Buffer, businessUnitId: string, description: string) {
    client.UploadAsset({ filename, file, business_unit_id: businessUnitId, description }, (error: any, response: any) => {
        if (error) {
            console.error('Erro ao fazer upload:', error);
        } else {
            console.log('Ativo criado com ID:', response.asset_id);
        }
    });
}
```

#### 3. Implementação no Backend (Rust)

**Instalação de Dependências:**

Adicione as dependências necessárias no `Cargo.toml`:

```toml
[dependencies]
tonic = "0.4"
prost = "0.7"
tokio = { version = "1", features = ["full"] }
```

**Exemplo de Implementação:**

```rust
use tonic::{transport::Server, Request, Response, Status};
use upload::upload_service_server::{UploadService, UploadServiceServer};
use upload::{UploadAssetRequest, UploadAssetResponse};

pub mod upload {
    tonic::include_proto!("upload");
}

#[derive(Default)]
pub struct MyUploadService {}

#[tonic::async_trait]
impl UploadService for MyUploadService {
    async fn upload_asset(
        &self,
        request: Request<UploadAssetRequest>,
    ) -> Result<Response<UploadAssetResponse>, Status> {
        let req = request.into_inner();

        // Processar a requisição e armazenar o arquivo
        let asset_id = "novo_id_do_ativo"; // Substitua com a lógica real

        let response = UploadAssetResponse {
            asset_id: asset_id.to_string(),
        };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let upload_service = MyUploadService::default();

    Server::builder()
        .add_service(UploadServiceServer::new(upload_service))
        .serve(addr)
        .await?;

    Ok(())
}
```

#### 4. Integração com Keycloak

**Validação do Token JWT no Backend:**

- Utilize bibliotecas como `jsonwebtoken` para validar tokens JWT no backend.
- Verifique a assinatura e o tempo de expiração do token.
- Extraia informações relevantes (como o ID do usuário) do token para autorização.

**Exemplo em Rust:**

```rust
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    // Outros campos relevantes
}

fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let decoding_key = DecodingKey::from_secret("seu-segredo".as_ref());
    let validation = Validation::new(Algorithm::HS256);

    let token_data = decode::<Claims>(&token, &decoding_key, &validation)?;

    Ok(token_data.claims)
}

// Use a função validate_token na implementação gRPC para validar o token
```

---

### Manual de Boas Práticas

1. **Consistência:**
   - Mantenha a consistência nas definições Protobuf, nomes de métodos e campos.
   - Utilize nomes claros e descritivos.

2. **Modularidade:**
   - Organize as definições Protobuf em arquivos separados por contexto funcional.
   - Evite definir tudo em um único arquivo grande.

3. **Versão:**
   - Inclua números de versão nos serviços e mensagens para facilitar a compatibilidade futura.
   - Exemplo: `message UploadAssetRequestV1 { ... }`

4. **Documentação:**
   - Documente cada campo e serviço Protobuf.
   - Utilize comentários para explicar a finalidade de cada campo.

5. **Segurança:**
   - Sempre valide tokens JWT no backend.
   - Verifique permissões e roles conforme necessário.

6. **Teste:**
   - Escreva testes unitários e de integração para verificar a funcionalidade de serviços gRPC.
   - Utilize mocks para testar diferentes cenários.

7. **Gerenciamento de Erros:**
   - Defina claramente como os erros serão retornados nas respostas gRPC.
   - Utilize códigos de status apropriados (`OK`, `NOT_FOUND`, `UNAUTHORIZED`, etc.).

8. **Manutenção:**
   - Revise e atualize as definições Protobuf conforme necessário.
   - Mantenha um changelog para rastrear mudanças nas definições.


