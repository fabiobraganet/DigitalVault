### Planejamento de Configuração do Realm do Keycloak para DigitalVault

**Data:** 6 de julho de 2024

---

### Visão Geral

Estas tabelas descrevem as configurações do realm no Keycloak para o projeto DigitalVault. O objetivo é garantir que o ambiente de desenvolvimento e teste tenha uma configuração completa e funcional para autenticação e autorização.

---

### Realm: *DigitalVault*

| Atributo              | Valor              |
|-----------------------|--------------------|
| Nome                  | DigitalVault       |
| Nome de Exibição      | DigitalVault       |
| Habilitado            | Sim                |
| SSL Obrigatório       | Apenas Externo     |
| Registro de Usuário   | Desativado         |
| Login com Email       | Habilitado         |
| Email como Nome de Usuário | Habilitado   |
| Lembrar de Mim        | Habilitado         |
| Verificar Email       | Habilitado         |
| Redefinir Senha       | Habilitado         |
| Editar Nome de Usuário | Desativado        |
| Idiomas Habilitados   | en, pt-BR          |

---

### Configurações do Realm

| Categoria       | Atributo                      | Valor                            |
|-----------------|-------------------------------|----------------------------------|
| Geral           | Nome de Exibição              | DigitalVault                    |
|                 | Nome de Exibição HTML         | DigitalVault                    |
|                 | URL Frontend                  | http://localhost:8080/auth/realms/DigitalVault |
| Temas           | Tema de Login                 | keycloak                        |
| Email           | Endereço de Remetente         | noreply@digitalvault.com        |
|                 | SSL Habilitado                | Desativado                      |
|                 | Host                          | smtp.digitalvault.com           |
|                 | Porta                         | 587                             |
|                 | Autenticação                  | Nome de Usuário e Senha         |
|                 | Nome de Usuário               | smtp-user                       |
|                 | Senha                         | smtp-password                   |
| Tokens          | Tempo de Vida do Token de Acesso | 5m                           |
|                 | Tempo de Vida do Token de Atualização | 30m                    |
|                 | Sessão SSO Inativa            | 30m                             |
|                 | Sessão SSO Máxima             | 2h                              |
|                 | Tempo de Vida do Token de Verificação de Email | 12h                |
|                 | Tempo de Vida do Token de Redefinição de Senha | 1h                 |

---

### Autenticação

| Método de Autenticação | Habilitado        |
|------------------------|-------------------|
| Username/Password      | Sim               |
| OTP                    | Opcional          |

---

### Políticas de Senha

| Atributo                | Valor              |
|-------------------------|--------------------|
| Tamanho Mínimo          | 8                  |
| Letras Maiúsculas       | 1                  |
| Letras Minúsculas       | 1                  |
| Dígitos                 | 1                  |
| Caracteres Especiais    | 1                  |
| Histórico de Senhas     | 3                  |
| Expiração de Senha      | 180 dias           |

---

### Clientes

| Atributo                    | Valor                                     |
|-----------------------------|-------------------------------------------|
| ID do Cliente               | digitalvault-frontend                     |
| Nome                        | DigitalVault Frontend                     |
| Habilitado                  | Sim                                       |
| Protocolo do Cliente        | openid-connect                            |
| Tipo de Acesso              | Público                                   |
| Fluxo Padrão Habilitado     | Sim                                       |
| Fluxo Implícito Habilitado  | Não                                       |
| Concessões de Acesso Direto Habilitadas | Sim                           |
| Contas de Serviço Habilitadas | Não                                     |
| Autorização Habilitada      | Não                                       |
| URIs de Redirecionamento Válidas | http://localhost:3000/*              |
| URL Base                    | http://localhost:3000                     |
| Origens Web                 | http://localhost:3000                     |
| Logout em Canal Frontal     | Habilitado                                |
| URL Raiz                    | http://localhost:3000                     |
| URL de Administração        |                                           |
| URIs de Redirecionamento    | http://localhost:3000/*                   |
| Origens Web                 | http://localhost:3000                     |

---

### Mapeadores de Protocolo

| Nome                   | Tipo      | Atributos                          | Reivindicação no Token |
|------------------------|-----------|------------------------------------|------------------------|
| username               | User Property | username                       | preferred_username     |
| email                  | User Property | email                          | email                  |
| roles                  | Realm Roles | roles                           | roles                  |
| custom_attribute       | User Attribute | custom_attribute              | custom_attribute       |

---

### Escopos do Cliente

| Nome                   | Tipo     | Atributos                | Mapeamentos                  |
|------------------------|----------|--------------------------|------------------------------|
| email                  | padrão   | Incluir no Escopo do Token | email                        |
| profile                | padrão   | Incluir no Escopo do Token | profile                      |
| roles                  | padrão   | Incluir no Escopo do Token | roles                        |
| digitalvault-specific  | opcional | Incluir no Escopo do Token | digitalvault-specific-mapper |

---

### Papéis do Realm

| Nome                   | Descrição                                  |
|------------------------|--------------------------------------------|
| admin                  | Papel com todos os privilégios administrativos |
| user                   | Papel de usuário regular com privilégios básicos |
| manager                | Papel com privilégios elevados para gerenciar recursos |

---

### Usuários

| Nome de Usuário        | Email                      | Senha      | Papéis               |
|------------------------|----------------------------|------------|----------------------|
| admin                  | admin@digitalvault.com     | admin123   | admin                |
| user1                  | user1@digitalvault.com     | user1234   | user                 |
| manager1               | manager1@digitalvault.com  | manager123 | manager              |

---

### Grupos

| Nome                   | Papéis                      | Membros                       |
|------------------------|-----------------------------|-------------------------------|
| Admin Group            | admin                       | admin                         |
| User Group             | user                        | user1                         |
| Manager Group          | manager                     | manager1                      |

