# DigitalVault
> Plataforma de gerenciamento de ativos digitais

---

### README - Frontend DigitalVault

**Data:** 6 de julho de 2024

---

# DigitalVault - Frontend

DigitalVault é uma plataforma robusta e segura para o gerenciamento de ativos digitais. Este repositório contém o código-fonte do frontend, desenvolvido utilizando React, Ant Design, Context API e React Query. A aplicação fornece uma interface de usuário moderna e acessível para o gerenciamento de ativos digitais.

## Visão Geral

O frontend do DigitalVault é responsável por fornecer uma interface de usuário intuitiva e eficiente, que permite aos usuários fazer upload, gerenciar e visualizar ativos digitais. A aplicação é integrada com Keycloak para autenticação e autorização, e se comunica com o backend via gRPC.

## Tecnologias Utilizadas

- **Linguagem:** TypeScript
- **Framework:** React
- **UI Library:** Ant Design
- **Gerenciamento de Estado:** Context API
- **Gerenciamento de Requisições:** React Query
- **Autenticação:** Keycloak
- **Infraestrutura:** Vite

## Estrutura do Projeto

```plaintext
digitalvault-frontend/
├── src/
│   ├── App.css
│   ├── App.tsx
│   ├── antd-theme.tsx
│   ├── contexts/
│   │   ├── ProfilerContext/
│   │   │   ├── index.tsx
│   │   │   ├── reducer.tsx
│   ├── components/
│   │   ├── drawers/
│   │   │   ├── DrawerLanguages.tsx
│   │   │   ├── DrawerThemes.tsx
│   │   ├── FlagIcon.tsx
│   ├── Home/
│   │   ├── Index.tsx
│   ├── i18n.tsx
│   ├── index.css
│   ├── main.tsx
│   ├── layouts/
│   │   ├── HeaderContentFooter.tsx
│   │   ├── HeaderContentSiderFooter.tsx
│   │   ├── HeaderSiderContentFooter.tsx
│   │   ├── HeaderSiderContentSiderFooter.tsx
│   │   ├── SiderHeaderContentFooter.tsx
│   ├── pages/
│   │   ├── Administration/
│   │   ├── Error/
│   │   │   ├── Error401.tsx
│   │   │   ├── Error403.tsx
│   │   │   ├── Error404.tsx
│   │   ├── Operation/
│   │   │   ├── index.tsx
│   │   │   ├── parts/
│   │   │   │   ├── HeaderCommand.tsx
│   │   │   │   ├── HeaderLocalBusiness.tsx
│   │   │   │   ├── HeaderLogo.tsx
│   │   │   │   ├── HeaderMenuIcons.tsx
│   │   │   ├── zones/
│   │   │   │   ├── ZoneContent.tsx
│   │   │   │   ├── ZoneFooter.tsx
│   │   │   │   ├── ZoneHeader.tsx
│   ├── routers/
│   │   ├── index.tsx
│   ├── types/
│   │   ├── CascadeOption.tsx
│   │   ├── ProfilerAction.tsx
│   │   ├── ProfilerState.tsx
│   │   ├── security-types.tsx
│   │   ├── ThemeType.tsx
│   ├── vite-env.d.ts
├── .env
├── .gitignore
├── index.html
├── package.json
├── tsconfig.json
├── vite.config.ts
```

## Configuração do Ambiente

### Pré-requisitos

- [Node.js](https://nodejs.org/)
- [Yarn](https://yarnpkg.com/) ou [NPM](https://www.npmjs.com/)

### Configuração

1. Instale as dependências:

```bash
yarn install
# ou
npm install
```

2. Configure as variáveis de ambiente no arquivo `.env`:

```plaintext
REACT_APP_KEYCLOAK_URL=http://localhost:6003/auth
REACT_APP_KEYCLOAK_REALM=digitalvault
REACT_APP_KEYCLOAK_CLIENT_ID=digitalvault-frontend
REACT_APP_API_URL=http://localhost:50051
```

3. Inicie a aplicação:

```bash
yarn dev
# ou
npm run dev
```

## Estrutura de Componentes

### Home Page

**Descrição:** Tela principal que agrupa todas as funcionalidades.

**Componentes:**
- **Menu de Navegação Superior**
- **Resumo de Ativos**
- **Upload de Ativos**
- **Gerenciamento de Ativos**

#### Menu de Navegação Superior

```plaintext
---------------------------------------------------------
| Logo | Unidade de Negócio: [Dropdown] | Logout         |
---------------------------------------------------------
| Resumo de Ativos | Upload de Ativos | Gerenciamento de Ativos |
---------------------------------------------------------
```

#### Resumo de Ativos

```plaintext
---------------------------------------------------------
|                  Resumo de Ativos                     |
---------------------------------------------------------
| Total de Ativos: 120                                  |
|-------------------------------------------------------|
| Atividades Recentes:                                  |
| - [Data] [Evento]                                     |
| - [Data] [Evento]                                     |
| - [Data] [Evento]                                     |
---------------------------------------------------------
```

#### Upload de Ativos

```plaintext
---------------------------------------------------------
|                  Upload de Ativos                     |
---------------------------------------------------------
| Drag-and-Drop Zone                                    |
| [Botão de Seleção de Arquivo]                         |
|-------------------------------------------------------|
| Nome do Arquivo: [Input Field]                        |
| Unidade de Negócio: [Selecionada Automaticamente]     |
| Descrição: [Text Area]                                |
|-------------------------------------------------------|
| [Botão de Envio]                                      |
---------------------------------------------------------
```

#### Gerenciamento de Ativos

```plaintext
---------------------------------------------------------
|              Gerenciamento de Ativos                  |
---------------------------------------------------------
| Busca: [Input Field] [Botão de Busca]                 |
| Filtros: [Unidade de Negócio] [Tipo de Arquivo] [Data]|
---------------------------------------------------------
| Tabela de Ativos                                      |
|-------------------------------------------------------|
| Nome        | Tipo       | Data de Upload | Unidade de Negócio | Ações    |
|-------------|------------|----------------|--------------------|----------|
| Arquivo 1   | Imagem     | 01/07/2024     | Unidade 1          | [Ver] [Editar] [Excluir] |
| Arquivo 2   | Documento  | 02/07/2024     | Unidade 2          | [Ver] [Editar] [Excluir] |
| Arquivo 3   | Vídeo      | 03/07/2024     | Unidade 1          | [Ver] [Editar] [Excluir] |
---------------------------------------------------------
```

