# Architecture Decision Record (ADR)

## Título: Decisões de Arquitetura para o Projeto DigitalVault

**Data:** 6 de julho de 2024

**Status:** Aceito

## Contexto:
DigitalVault é uma plataforma de gerenciamento de ativos digitais que permite aos usuários fazer upload, organizar e compartilhar arquivos de forma segura e eficiente. A plataforma deve ser integrada com Keycloak para autenticação, MinIO para armazenamento de arquivos, PostgreSQL para banco de dados, e RabbitMQ para gerenciamento de eventos de domínio. A comunicação entre o frontend e o backend será realizada via gRPC.

---

## Decisão 1: Utilização de Keycloak para Autenticação e Autorização

**Contexto:**
O sistema precisa de um mecanismo robusto para autenticar e autorizar usuários, gerenciar sessões, e definir diferentes níveis de acesso com base em papéis.

**Decisão:**
Optamos por utilizar Keycloak como a solução de autenticação e autorização. Keycloak é uma ferramenta de código aberto que oferece autenticação e autorização com suporte a SSO, OAuth2, OpenID Connect e outras funcionalidades avançadas.

**Justificativa:**

- Keycloak oferece uma solução completa e madura para gerenciamento de identidade.
- Suporte nativo para OAuth2 e OpenID Connect facilita a integração com outros serviços.
- Ferramentas de administração web-friendly.
- Comunidade ativa e documentação robusta.

**Consequências:**

- Necessidade de configurar e manter o Keycloak como parte da infraestrutura.
- Implementação de integração com Keycloak no frontend e backend.

---

## Decisão 2: Uso de gRPC para Comunicação entre Frontend e Backend

**Contexto:**
A comunicação entre o frontend e o backend deve ser rápida, segura e eficiente para suportar operações de upload e gerenciamento de ativos.

**Decisão:**
Optamos por utilizar gRPC como o protocolo de comunicação entre o frontend e o backend.

**Justificativa:**

- gRPC oferece comunicação eficiente e de alta performance utilizando HTTP/2.
- Suporte a definições de serviço com contratos claros através de arquivos `.proto`.
- Suporte a múltiplas linguagens de programação, facilitando a integração entre frontend em React e backend em Rust.
- Recursos como streaming bidirecional e comunicação segura (TLS).

**Consequências:**

- Necessidade de definir e manter arquivos `.proto` para serviços gRPC.
- Implementação e manutenção de servidores e clientes gRPC.

---

## Decisão 3: Armazenamento de Arquivos com MinIO

**Contexto:**
O sistema deve fornecer armazenamento seguro e escalável para arquivos digitais, com suporte a operações de upload, download e gerenciamento.

**Decisão:**
Optamos por utilizar MinIO como solução de armazenamento de arquivos.

**Justificativa:**

- MinIO é uma solução de armazenamento de objetos compatível com S3.
- Fácil de integrar com aplicações via API S3.
- Escalável e pode ser auto-hospedado, oferecendo flexibilidade de implantação.
- Suporte a alta disponibilidade e redundância.

**Consequências:**

- Necessidade de configurar e manter o MinIO como parte da infraestrutura.
- Implementação de integração com MinIO no backend.

---

## Decisão 4: Uso de PostgreSQL para Armazenamento de Metadados

**Contexto:**
O sistema precisa armazenar metadados dos arquivos, informações dos usuários e outros dados estruturados de forma eficiente e segura.

**Decisão:**
Optamos por utilizar PostgreSQL como o banco de dados relacional para armazenar os metadados.

**Justificativa:**

- PostgreSQL é um banco de dados relacional robusto e amplamente utilizado.
- Suporte a transações ACID garante a integridade dos dados.
- Capacidade de escalar horizontalmente com replicação.
- Extensibilidade com suporte a JSON e outras funcionalidades avançadas.

**Consequências:**

- Necessidade de configurar e manter o PostgreSQL como parte da infraestrutura.
- Implementação de repositórios e integração com PostgreSQL no backend utilizando Diesel ORM.

---

## Decisão 5: Gerenciamento de Eventos com RabbitMQ

**Contexto:**
O sistema deve gerenciar e distribuir eventos de domínio para notificar alterações nos ativos, como adição, atualização e exclusão.

**Decisão:**
Optamos por utilizar RabbitMQ como o sistema de mensageria para gerenciamento de eventos de domínio.

**Justificativa:**

- RabbitMQ é uma solução de mensageria robusta e amplamente adotada.
- Suporte a diversos padrões de mensagem, incluindo pub/sub.
- Alta disponibilidade e capacidade de escalabilidade.
- Ferramentas de administração e monitoramento eficazes.

**Consequências:**

- Necessidade de configurar e manter o RabbitMQ como parte da infraestrutura.
- Implementação de publicação e assinatura de eventos no backend.

---

## Decisão 6: Estrutura de Projeto Monorepo

**Contexto:**
Para facilitar o desenvolvimento e manutenção do projeto, a estrutura do repositório deve ser organizada de forma clara, permitindo que frontend e backend coexistam no mesmo repositório.

**Decisão:**
Optamos por utilizar uma estrutura de projeto monorepo onde frontend e backend estão no mesmo repositório GitHub, organizados em diretórios separados.

**Justificativa:**

- Facilita a gestão de dependências e versões.
- Simplifica o fluxo de desenvolvimento e integração contínua.
- Melhora a colaboração entre desenvolvedores frontend e backend.
- Possibilita um controle centralizado de configuração e scripts de build/deploy.

**Consequências:**

- Necessidade de configurar o repositório para suportar monorepo.
- Implementação de scripts de automação e integração contínua.

---

## Decisão 7: Utilização do Ant Design para Interface de Usuário

**Contexto:**
A interface de usuário deve ser moderna, acessível e fácil de usar, com componentes reutilizáveis e consistentes.

**Decisão:**
Optamos por utilizar Ant Design como a biblioteca de componentes UI para o frontend.

**Justificativa:**

- Ant Design oferece uma ampla gama de componentes UI bem documentados.
- Consistência visual e acessibilidade garantida.
- Integração fácil com React.
- Suporte a temas personalizados e internacionalização.

**Consequências:**

- Necessidade de configurar e manter Ant Design no projeto frontend.
- Treinamento de desenvolvedores para utilizar a biblioteca de forma eficaz.

