### Documentação do Projeto Frontend - DigitalVault

**Data:** 6 de julho de 2024

---

### Visão Geral

O frontend do DigitalVault é uma aplicação web desenvolvida utilizando React, Ant Design, Context API e React Query. A aplicação fornece uma interface de usuário moderna e acessível para o gerenciamento de ativos digitais. Esta documentação descreve a estrutura do projeto, os componentes principais e os wireframes das telas.

---

### Estrutura do Projeto

**Árvore Completa do Sistema de Arquivos**

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
│   │   ├── Home/
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

**Descrição dos Diretórios e Arquivos**

- **.env**: Configuração de variáveis de ambiente.
- **.gitignore**: Arquivos e diretórios a serem ignorados pelo Git.
- **index.html**: Página HTML principal.
- **package.json**: Configuração do NPM, dependências e scripts.
- **tsconfig.json**: Configuração do TypeScript.
- **vite.config.ts**: Configuração do Vite.
- **App.css**: Estilos globais do componente principal.
- **App.tsx**: Componente principal da aplicação.
- **antd-theme.tsx**: Configuração de temas do Ant Design.
- **contexts/**: Contextos para gerenciamento de estado global.
- **components/**: Componentes reutilizáveis.
- **Home/**: Componente principal da página inicial.
- **i18n.tsx**: Configuração do i18next para internacionalização.
- **index.css**: Estilos globais.
- **main.tsx**: Ponto de entrada da aplicação React.
- **layouts/**: Componentes de layout.
- **pages/**: Páginas da aplicação.
- **routers/**: Configuração das rotas.
- **types/**: Definição de tipos.
- **vite-env.d.ts**: Tipos de referência para o Vite.

---

### Wireframes das Telas

#### Tela de Login

**Descrição:** Utilização do Keycloak para autenticação.

**Componentes:**
- Botão de Login: Redireciona para o Keycloak para login.

```
---------------------------------------------------------
|                          LOGIN                        |
|-------------------------------------------------------|
| [Login with Keycloak]                                 |
---------------------------------------------------------
```

#### Tela Home

**Descrição:** Tela principal que agrupa todas as funcionalidades.

**Componentes:**
- Menu de Navegação Superior
- Botão de logout.
- Links rápidos para seções (Resumo de Ativos, Upload, Gerenciamento).
- Resumo de Ativos
- Total de ativos.
- Atividades recentes (logs de eventos de domínio).
- Upload de Ativos
- Formulário de upload de arquivos (drag-and-drop e seleção de arquivo).
- Campos para associar o arquivo a uma unidade de negócio (BusinessUnitId) e outras metainformações.
- Botão de envio.
- Gerenciamento de Ativos
- Tabela de ativos (listagem de arquivos com detalhes como nome, tipo, data de upload, unidade de negócio).
- Botões de ação (visualizar, editar, excluir).
- Filtro e busca por ativos.

##### Menu de Navegação Superior

```
---------------------------------------------------------
| Logo | Unidade de Negócio: [Dropdown] | Logout         |
---------------------------------------------------------
| Resumo de Ativos | Upload de Ativos | Gerenciamento de Ativos |
---------------------------------------------------------
```

##### Resumo de Ativos

```
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

##### Upload de Ativos

**Descrição:** Permite o upload de novos arquivos diretamente da Home, utilizando o valor da unidade de negócio selecionada no topo da tela.

```
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

##### Gerenciamento de Ativos

```
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

