### Estrutura do Repositório GitHub

Para manter o backend e o frontend no mesmo repositório GitHub de forma organizada e seguindo boas práticas, é importante definir uma estrutura de pastas clara. Aqui está uma sugestão de estrutura:

```
root/
|-- frontend/
|   |-- src/
|   |-- public/
|-- backend/
|   |-- src/
|-- proto/
|-- docker/
|-- .gitignore
|-- README.md
```

### Descrição dos Diretórios e Arquivos

#### 1. Frontend

- **frontend/src/**: Contém todo o código-fonte do frontend.
- **frontend/public/**: Arquivos públicos estáticos.

#### 2. Backend

- **backend/src/**: Contém todo o código-fonte do backend.

#### 3. Proto

- **proto/**: Contém os arquivos `.proto` para definir os serviços gRPC.

#### 4. Docker

- **docker/**: Contém arquivos de configuração para Docker.
  - **docker-compose.yml**: Arquivo de configuração do Docker Compose 

#### 5. Raiz do Repositório

- **.gitignore**: Arquivo para especificar arquivos e diretórios a serem ignorados pelo Git.
- **README.md**: Arquivo de documentação do projeto.

### Passos para Configuração e Execução

#### 1. Clonar o Repositório

```bash
git clone https://github.com/fabiobraganet/DigitalVault.git
cd DigitalVault
```

#### 2. Configurar o Frontend

```bash
cd frontend
npm install
```

#### 3. Configurar o Backend

```bash
cd backend
cargo build
```

#### 4. Configurar o Docker

```bash
cd docker
docker-compose up -d
```

### Fluxo de Trabalho

1. **Desenvolvimento no Frontend**
   - Utilizando React, Ant Design, Context API e React Query para construir a interface de usuário.
   - Autenticação via Keycloak e comunicação com o backend via gRPC.

2. **Desenvolvimento no Backend**
   - Implementando serviços em Rust utilizando Actix-web, Diesel, MinIO e RabbitMQ.
   - Serviços gRPC definidos nos arquivos `.proto`.

3. **Integração e Testes**
   - Verificar a comunicação entre frontend e backend utilizando gRPC.
   - Testar funcionalidades de upload e gerenciamento de ativos.

4. **Deploy**
   - Utilizar Docker e Docker Compose para orquestrar os contêineres.
   - Configurar variáveis de ambiente e outros parâmetros de configuração para produção.
