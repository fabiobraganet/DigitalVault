### Documento de Requisitos de Negócio

#### Projeto: DigitalVault

**Descrição Geral:**
DigitalVault é uma plataforma de gerenciamento de ativos digitais que permite aos usuários fazer upload, organizar e compartilhar arquivos de forma segura e eficiente. A plataforma é integrada com Keycloak para autenticação, MinIO para armazenamento de arquivos, PostgreSQL para banco de dados, e RabbitMQ para gerenciamento de eventos de domínio. A comunicação entre o frontend e o backend será realizada via gRPC.

---

### Requisitos de Negócio

#### 1. Autenticação e Autorização
**Descrição:**
- O sistema deve permitir que os usuários façam login utilizando o Keycloak.
- Os usuários devem ser autenticados via tokens JWT.
- Diferentes níveis de acesso devem ser definidos com base em papéis (admin, editor, visualizador).

**Funcionalidades:**
- Integração com Keycloak.
- Validação de tokens JWT.
- Gestão de sessões de usuários.

#### 2. Upload de Ativos
**Descrição:**
- Usuários devem poder fazer upload de arquivos digitais.
- Os arquivos devem ser associados a uma unidade de negócio específica.
- Metadados, como nome do arquivo e descrição, devem ser fornecidos durante o upload.

**Funcionalidades:**
- Área de upload com suporte a drag-and-drop e seleção de arquivo.
- Formulário para inserção de metadados.
- Associação automática da unidade de negócio selecionada.

#### 3. Organização de Ativos
**Descrição:**
- Os ativos devem ser organizados por unidades de negócio.
- Usuários devem poder visualizar, editar e excluir ativos.
- Ativos devem ter metadados associados para facilitar a organização e busca.

**Funcionalidades:**
- Tabela de listagem de ativos com detalhes como nome, tipo, data de upload, unidade de negócio.
- Filtros e busca para localizar ativos específicos.
- Ações de visualização, edição e exclusão de ativos.

#### 4. Armazenamento Seguro de Arquivos
**Descrição:**
- Arquivos devem ser armazenados de forma segura no MinIO.
- O sistema deve garantir a integridade e disponibilidade dos arquivos.

**Funcionalidades:**
- Upload e download de arquivos para e do MinIO.
- Gestão de buckets e objetos no MinIO.

#### 5. Eventos de Domínio
**Descrição:**
- O sistema deve notificar eventos importantes, como a adição, atualização ou exclusão de ativos.
- Eventos devem ser gerenciados e distribuídos via RabbitMQ.

**Funcionalidades:**
- Publicação de eventos de domínio no RabbitMQ.
- Assinatura e processamento de eventos de domínio.

#### 6. Comunicação Segura
**Descrição:**
- A comunicação entre o frontend e o backend deve ser segura e eficiente.
- Utilização de gRPC para chamadas rápidas e seguras.

**Funcionalidades:**
- Definição de serviços e mensagens gRPC.
- Implementação de servidor gRPC no backend e cliente gRPC no frontend.

#### 7. Banco de Dados
**Descrição:**
- O sistema deve utilizar PostgreSQL para armazenar metadados dos arquivos e informações dos usuários.
- O banco de dados deve ser estruturado de forma a suportar consultas eficientes e escaláveis.

**Funcionalidades:**
- Tabelas para usuários, unidades de negócio e metadados dos arquivos.
- Migrações gerenciadas pelo Diesel.

#### 8. Interface de Usuário
**Descrição:**
- A interface deve ser moderna, acessível e fácil de usar.
- Utilização do Ant Design para consistência e acessibilidade.

**Funcionalidades:**
- Componentes reutilizáveis do Ant Design.
- Navegação clara e intuitiva.
- Resumo de ativos, upload de arquivos e gerenciamento de ativos na tela principal.

---

### Casos de Uso

1. **Usuário Faz Login**
   - Pré-condições: O usuário possui uma conta no Keycloak.
   - Fluxo principal:
     - O usuário acessa a página de login.
     - O usuário é redirecionado para o Keycloak para autenticação.
     - Após autenticação bem-sucedida, o usuário é redirecionado para a tela Home.
   - Pós-condições: O usuário está autenticado e pode acessar as funcionalidades da plataforma.

2. **Usuário Faz Upload de Arquivo**
   - Pré-condições: O usuário está autenticado e selecionou uma unidade de negócio.
   - Fluxo principal:
     - O usuário acessa a seção de upload de ativos.
     - O usuário arrasta e solta o arquivo ou usa o botão de seleção de arquivo.
     - O usuário preenche os metadados do arquivo.
     - O usuário clica no botão de envio.
     - O sistema faz upload do arquivo para o MinIO e armazena os metadados no PostgreSQL.
     - Um evento de adição de ativo é publicado no RabbitMQ.
   - Pós-condições: O arquivo é armazenado e os metadados são salvos. O evento de adição é publicado.

3. **Usuário Visualiza Ativos**
   - Pré-condições: O usuário está autenticado.
   - Fluxo principal:
     - O usuário acessa a seção de gerenciamento de ativos.
     - O sistema exibe uma tabela com os ativos disponíveis.
     - O usuário pode usar a busca e filtros para localizar ativos específicos.
   - Pós-condições: O usuário visualiza os ativos disponíveis.

4. **Usuário Edita Ativo**
   - Pré-condições: O usuário está autenticado e tem permissão de edição.
   - Fluxo principal:
     - O usuário acessa a seção de gerenciamento de ativos.
     - O usuário seleciona o ativo a ser editado.
     - O usuário altera os metadados do ativo.
     - O usuário salva as alterações.
     - O sistema atualiza os metadados no PostgreSQL.
     - Um evento de atualização de ativo é publicado no RabbitMQ.
   - Pós-condições: Os metadados do ativo são atualizados e o evento de atualização é publicado.

5. **Usuário Exclui Ativo**
   - Pré-condições: O usuário está autenticado e tem permissão de exclusão.
   - Fluxo principal:
     - O usuário acessa a seção de gerenciamento de ativos.
     - O usuário seleciona o ativo a ser excluído.
     - O usuário confirma a exclusão.
     - O sistema remove o arquivo do MinIO e os metadados do PostgreSQL.
     - Um evento de exclusão de ativo é publicado no RabbitMQ.
   - Pós-condições: O arquivo e seus metadados são removidos e o evento de exclusão é publicado.

---

### Requisitos Funcionais e Não-Funcionais

**Funcionais:**
1. Integração com Keycloak para autenticação e autorização.
2. Upload de arquivos com metadados.
3. Armazenamento seguro de arquivos no MinIO.
4. Gerenciamento de ativos (visualização, edição, exclusão).
5. Publicação e processamento de eventos de domínio via RabbitMQ.
6. Comunicação entre frontend e backend via gRPC.
7. Interface de usuário responsiva e acessível utilizando Ant Design.
8. Banco de dados PostgreSQL para armazenamento de metadados e informações dos usuários.

**Não-Funcionais:**
1. Segurança: Garantir comunicação segura entre frontend e backend.
2. Escalabilidade: Suporte para um grande volume de arquivos e usuários.
3. Performance: Respostas rápidas e eficientes.
4. Manutenibilidade: Código limpo e bem estruturado seguindo princípios de Arquitetura Limpa e SOLID.
5. Usabilidade: Interface intuitiva e fácil de usar.

