# DigitalVault
>Plataforma de gerenciamento de ativos digitais

---

### Documento do Projeto DigitalVault

**Data:** 6 de julho de 2024

---

#### ver também

[DRN](./docs/DRN.md) | [ADR](./docs/ADR.md) | [Infraestrutura](./docs/Infra.md) | [Repositório](./docs//Repo.md) | [Contas](./docs/Accounts-Plan.md) | [Frontend](./docs/Frontend.md) | [Backend](./docs/Backend.md)

---

### Visão Geral do Projeto DigitalVault

**DigitalVault** é uma plataforma de gerenciamento de ativos digitais que permite aos usuários fazer upload, organizar e compartilhar arquivos de forma segura e eficiente. O projeto é desenvolvido com uma arquitetura robusta e moderna, utilizando tecnologias como React para o frontend e Rust para o backend, e é integrado com várias ferramentas externas, como Keycloak, MinIO, PostgreSQL e RabbitMQ.

---

### Escopo Atual

#### Funcionalidades Principais

1. **Autenticação e Autorização**
   - **Descrição:** Utilização do Keycloak para gerenciar a autenticação e autorização dos usuários.
   - **Características:**
     - Login com email e senha.
     - Suporte a múltiplos papéis (admin, user, manager).
     - Sessões de usuário seguras com tokens JWT.

2. **Upload de Ativos**
   - **Descrição:** Permite que os usuários façam upload de arquivos digitais.
   - **Características:**
     - Suporte a upload via drag-and-drop e seleção de arquivos.
     - Metadados associados a cada arquivo (nome, descrição, unidade de negócio).
     - Armazenamento seguro dos arquivos no MinIO.

3. **Organização de Ativos**
   - **Descrição:** Organização de arquivos por unidades de negócio.
   - **Características:**
     - Associação de arquivos a unidades de negócio específicas.
     - Visualização e busca de arquivos organizados por filtros e categorias.

4. **Gerenciamento de Ativos**
   - **Descrição:** Funções de visualização, edição e exclusão de arquivos.
   - **Características:**
     - Listagem de ativos com detalhes como nome, tipo, data de upload e unidade de negócio.
     - Funcionalidades de edição de metadados e exclusão de arquivos.
     - Logs de atividades e eventos de domínio associados às operações.

5. **Armazenamento Seguro**
   - **Descrição:** Utilização do MinIO para armazenamento seguro de arquivos.
   - **Características:**
     - Integração com a API S3 para operações de upload, download e gerenciamento de arquivos.
     - Alta disponibilidade e redundância de dados.

6. **Gerenciamento de Eventos**
   - **Descrição:** Utilização do RabbitMQ para gerenciamento de eventos de domínio.
   - **Características:**
     - Publicação de eventos de adição, atualização e exclusão de ativos.
     - Assinatura e processamento de eventos para garantir a consistência dos dados.

7. **Interface de Usuário**
   - **Descrição:** Interface moderna e acessível desenvolvida com React e Ant Design.
   - **Características:**
     - Layouts flexíveis e componentes reutilizáveis.
     - Suporte a temas claro e escuro.
     - Internacionalização (i18n) com suporte inicial para inglês e português.

---

### Funcionalidades Futuras

#### 1. Controle de Versão de Ativos
- **Descrição:** Implementação de funcionalidades para controle de versão dos arquivos.
- **Características:**
  - Histórico de versões de cada arquivo.
  - Possibilidade de restaurar versões anteriores.
  - Comparação entre diferentes versões de um mesmo arquivo.

#### 2. Colaboração em Tempo Real
- **Descrição:** Funcionalidades para colaboração em tempo real entre usuários.
- **Características:**
  - Comentários em arquivos.
  - Anotações e marcações colaborativas.
  - Notificações em tempo real sobre mudanças e comentários.

#### 3. Integração com Outros Serviços de Armazenamento
- **Descrição:** Integração com outros serviços de armazenamento como Google Drive, Dropbox e OneDrive.
- **Características:**
  - Importação e exportação de arquivos entre DigitalVault e outros serviços.
  - Sincronização automática de arquivos.

#### 4. Funcionalidades Avançadas de Busca
- **Descrição:** Melhorias nas capacidades de busca e filtragem de arquivos.
- **Características:**
  - Busca por conteúdo dentro dos arquivos (OCR para imagens e PDF).
  - Filtros avançados baseados em metadados, data, tipo de arquivo, etc.
  - Busca por tags e categorias personalizadas.

#### 5. Relatórios e Análises
- **Descrição:** Ferramentas para geração de relatórios e análises de uso.
- **Características:**
  - Relatórios de atividade e uso dos arquivos.
  - Análises de acesso e download de arquivos.
  - Geração de relatórios personalizados com gráficos e estatísticas.

#### 6. Funcionalidades de Segurança Avançadas
- **Descrição:** Melhorias nas funcionalidades de segurança e conformidade.
- **Características:**
  - Auditoria de acesso e modificações de arquivos.
  - Criptografia de arquivos em repouso.
  - Conformidade com normas e regulamentações de segurança.

---
---

### Plano de Projeto - DigitalVault

**Data de Início:** 6 de julho de 2024  
**Data de Término Estimada:** 6 de outubro de 2024  
**Duração Total:** 3 meses

---

### Visão Geral

O projeto DigitalVault é uma plataforma de gerenciamento de ativos digitais, integrando autenticação e autorização com Keycloak, armazenamento de arquivos com MinIO, banco de dados PostgreSQL, mensageria com RabbitMQ e comunicação via gRPC. O projeto será desenvolvido utilizando uma abordagem modular e seguindo princípios de Arquitetura Limpa e SOLID.

---

### Estrutura do Projeto

#### Marcos Principais

1. **Configuração Inicial e Ambiente**
2. **Desenvolvimento do Backend**
3. **Desenvolvimento do Frontend**
4. **Integração e Testes**
5. **Documentação e Deploy**

---

### Cronograma e Estimativa de Horas

| Marco                           | Atividade                                         | Descrição                                                               | Duração (Horas) |
|---------------------------------|---------------------------------------------------|-------------------------------------------------------------------------|-----------------|
| **Configuração Inicial e Ambiente** |                                                   |                                                                         |                 |
|                                 | Configuração do Keycloak                          | Configurar o realm, clientes, papéis e usuários no Keycloak              | 16              |
|                                 | Configuração do Docker                            | Configurar Docker e Docker Compose para Keycloak, MinIO, PostgreSQL, RabbitMQ | 12              |
|                                 | Configuração do Ambiente de Desenvolvimento       | Configurar ambiente de desenvolvimento local e repositórios GitHub       | 8               |
| **Total Configuração Inicial**  |                                                   |                                                                         | **36**          |
| **Desenvolvimento do Backend**  |                                                   |                                                                         |                 |
|                                 | Estrutura do Projeto Backend                      | Criar estrutura básica do projeto backend em Rust                        | 8               |
|                                 | Implementação de Entidades e Eventos de Domínio   | Desenvolver entidades e eventos de domínio                               | 16              |
|                                 | Implementação de Serviços                         | Desenvolver serviços para upload e gerenciamento de ativos               | 24              |
|                                 | Integração com MinIO                              | Implementar cliente MinIO e operações de armazenamento                   | 16              |
|                                 | Integração com PostgreSQL                         | Implementar repositórios e integração com PostgreSQL                     | 16              |
|                                 | Integração com RabbitMQ                           | Implementar publicador e assinante de eventos                            | 12              |
|                                 | Implementação de gRPC                             | Desenvolver e configurar servidores gRPC                                 | 16              |
|                                 | Testes e Validação                                | Escrever e executar testes unitários e de integração                     | 24              |
| **Total Desenvolvimento Backend** |                                                   |                                                                         | **132**         |
| **Desenvolvimento do Frontend** |                                                   |                                                                         |                 |
|                                 | Estrutura do Projeto Frontend                     | Criar estrutura básica do projeto frontend em React                      | 8               |
|                                 | Configuração de Ant Design                        | Configurar e integrar Ant Design no projeto                              | 8               |
|                                 | Desenvolvimento de Componentes Reutilizáveis      | Desenvolver componentes de interface reutilizáveis                       | 24              |
|                                 | Desenvolvimento de Páginas                        | Desenvolver páginas principais (Home, Login, Upload, Gerenciamento)      | 32              |
|                                 | Implementação de Context API e React Query        | Implementar Context API para estado global e React Query para requisições | 16              |
|                                 | Integração com Keycloak                           | Implementar autenticação e autorização com Keycloak                      | 16              |
|                                 | Testes e Validação                                | Escrever e executar testes unitários e de integração                     | 16              |
| **Total Desenvolvimento Frontend** |                                                  |                                                                         | **120**         |
| **Integração e Testes**         |                                                   |                                                                         |                 |
|                                 | Integração Frontend e Backend                     | Integrar frontend e backend via gRPC                                     | 16              |
|                                 | Testes de Integração Completa                     | Executar testes de integração completa entre frontend e backend          | 24              |
|                                 | Ajustes e Correções de Bugs                       | Realizar ajustes e correções identificadas durante os testes             | 16              |
| **Total Integração e Testes**   |                                                   |                                                                         | **56**          |
| **Documentação e Deploy**       |                                                   |                                                                         |                 |
|                                 | Documentação do Projeto                           | Escrever documentação técnica detalhada para backend e frontend          | 24              |
|                                 | Preparação para Deploy                            | Preparar scripts e configurações para deploy                             | 16              |
|                                 | Deploy em Ambiente de Testes                      | Realizar deploy em ambiente de testes                                    | 8               |
|                                 | Ajustes Finais e Deploy em Produção               | Realizar ajustes finais e deploy em produção                             | 16              |
| **Total Documentação e Deploy** |                                                   |                                                                         | **64**          |
| **Total Geral do Projeto**      |                                                   |                                                                         | **408**         |

---

.

**1. Introdução**

O projeto DigitalVault visa criar uma plataforma robusta e segura para o gerenciamento de ativos digitais. Este documento detalha o cronograma, estimativa de horas e atividades necessárias para completar o projeto com sucesso.

**2. Objetivos do Projeto**

- Desenvolver um backend escalável e seguro utilizando Rust, MinIO, PostgreSQL e RabbitMQ.
- Criar um frontend moderno e acessível utilizando React, Ant Design, Context API e React Query.
- Integrar autenticação e autorização segura utilizando Keycloak.
- Garantir a comunicação eficiente entre frontend e backend via gRPC.
- Documentar e implementar processos de deploy para ambientes de teste e produção.

**3. Estrutura do Projeto**

O projeto está dividido em quatro marcos principais:
1. Configuração Inicial e Ambiente
2. Desenvolvimento do Backend
3. Desenvolvimento do Frontend
4. Integração e Testes
5. Documentação e Deploy

**4. Metodologia**

Utilizaremos uma metodologia ágil, com sprints semanais para acompanhar o progresso e ajustar o planejamento conforme necessário. Cada sprint incluirá reuniões de planejamento, revisão e retrospectiva para garantir a comunicação eficiente e a entrega contínua de valor.

**5. Cronograma e Estimativa de Horas**

O cronograma detalhado e a estimativa de horas foram descritos na tabela acima, com um total de 408 horas distribuídas ao longo de três meses.

**6. Equipe do Projeto**

A equipe do projeto incluirá:
- Desenvolvedores Backend
- Desenvolvedores Frontend
- Engenheiro DevOps
- Gerente de Projeto
- Analista de Qualidade

**7. Ferramentas e Tecnologias**

- **Backend:** Rust, MinIO, PostgreSQL, RabbitMQ, gRPC
- **Frontend:** React, Ant Design, Context API, React Query
- **Autenticação:** Keycloak
- **Infraestrutura:** Docker, Docker Compose
- **Gerenciamento do Projeto:** JIRA, Confluence

**8. Riscos e Mitigações**

- **Risco:** Problemas de integração entre frontend e backend.
  - **Mitigação:** Realizar integrações e testes frequentes durante o desenvolvimento.
- **Risco:** Problemas de performance e escalabilidade.
  - **Mitigação:** Implementar testes de carga e otimizações contínuas.

**9. Conclusão**

O projeto DigitalVault está planejado para ser uma plataforma robusta e segura para o gerenciamento de ativos digitais. Com uma abordagem modular e princípios de Arquitetura Limpa, garantimos a manutenibilidade e escalabilidade do sistema. Este documento serve como um guia detalhado para a execução bem-sucedida do projeto, garantindo a entrega dentro do prazo e do orçamento estimado.

---

**Acompanhamento e Revisão**

O progresso do projeto será revisado semanalmente em reuniões de sprint, com relatórios de status enviados ao final de cada sprint. Ajustes no cronograma e na alocação de recursos serão feitos conforme necessário para garantir a entrega bem-sucedida do projeto.

