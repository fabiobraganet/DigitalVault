# DigitalVault 

> Documentação de Infraestrutura

---

**Bem-vindo ao DigitalVault**, a plataforma inovadora para gerenciamento de ativos digitais. Este documento fornece uma visão detalhada dos requisitos de infraestrutura, tanto para implementação local quanto na nuvem, e apresenta um plano de escala robusto para volumes críticos. Vamos garantir que sua experiência com o DigitalVault seja otimizada e empolgante!

## Sumário

- [Requisitos Mínimos](#requisitos-mínimos)
- [Requisitos para Nuvem](#requisitos-para-nuvem)
- [Plano de Escala para Volumes Críticos](#plano-de-escala-para-volumes-críticos)

---

## Requisitos Mínimos

Para garantir o funcionamento eficiente do DigitalVault, é necessário atender aos seguintes requisitos mínimos:

### Hardware

<img src="https://cdn-icons-png.flaticon.com/512/6410/6410301.png" width="64" />

.
- **Processador**: Intel i5 ou equivalente
- **Memória RAM**: 8 GB
- **Armazenamento**: 100 GB de SSD
- **Rede**: Conexão de internet estável com pelo menos 10 Mbps de velocidade

### Software

<img src="https://cdn-icons-png.flaticon.com/512/1674/1674721.png" width="64" />

.
- **Sistema Operacional**: Ubuntu 20.04 LTS ou superior, CentOS 7 ou superior
- **Docker**: Versão 20.10 ou superior
- **Docker Compose**: Versão 1.27 ou superior
- **Git**: Versão 2.20 ou superior

### Configuração de Rede

<img src="https://cdn-icons-png.flaticon.com/512/3732/3732949.png" width="64" />

.
- **Portas**: Abertura das portas 5432 (PostgreSQL), 8080 (Keycloak), 9000 e 9001 (Minio), 5672 e 15672 (RabbitMQ)
- **Segurança**: Configurações básicas de firewall para proteger os serviços

---

## Requisitos para Nuvem

Para hospedar o DigitalVault na nuvem, os seguintes requisitos são recomendados:

### Provedor de Nuvem

<img src="https://cdn-icons-png.flaticon.com/512/7414/7414941.png" width="64" />

.

- **AWS**, **Google Cloud Platform (GCP)**, **Microsoft Azure**

### Configuração de Servidor

<img src="https://cdn-icons-png.flaticon.com/512/6584/6584876.png " width="64" />

.
- **Instância**: Mínimo t3.medium (AWS) ou equivalente
- **Memória RAM**: 16 GB
- **Armazenamento**: 200 GB de SSD com provisionamento de IOPS conforme necessário
- **Rede**: VPC configurada com sub-redes públicas e privadas, balanceador de carga (opcional para alta disponibilidade)

### Serviços Gerenciados (opcionais)

<img src="https://cdn-icons-png.flaticon.com/512/10218/10218826.png " width="64" />

.
- **Banco de Dados**: Amazon RDS para PostgreSQL ou Cloud SQL para PostgreSQL
- **Armazenamento de Objetos**: Amazon S3 ou Google Cloud Storage
- **Sistema de Mensagens**: Amazon MQ ou Google Cloud Pub/Sub

### Configuração de Segurança

<img src="https://cdn-icons-png.flaticon.com/512/857/857034.png " width="64">

.
- **IAM**: Configuração de políticas de acesso para usuários e serviços
- **Segurança de Rede**: Configuração de grupos de segurança e regras de firewall para proteger os serviços
- **Backup**: Políticas de backup automático para banco de dados e armazenamento

---

## Plano de Escala para Volumes Críticos

Para lidar com volumes críticos e garantir a escalabilidade do DigitalVault, siga este plano de escala:

### Escala Horizontal

<img src="https://cdn-icons-png.flaticon.com/512/9742/9742768.png " width="64">

- **Serviços Docker**: Utilize o Docker Swarm ou Kubernetes para orquestração de contêineres e escalabilidade horizontal.
  - **Replicas**: Configure réplicas de serviços críticos como Keycloak, Minio e RabbitMQ.
  - **Auto Scaling**: Utilize auto scaling para ajustar automaticamente o número de réplicas com base na carga de trabalho.

### Escala Vertical

<img src="   https://cdn-icons-png.flaticon.com/512/5168/5168178.png " width="64">

.
- **Recursos de Servidor**: Aumente a capacidade dos servidores adicionando mais CPU e memória conforme necessário.
  - **Monitoramento**: Utilize ferramentas como Prometheus e Grafana para monitorar o desempenho e a utilização de recursos.
  - **Ajustes de Configuração**: Otimize a configuração dos serviços Docker para utilizar melhor os recursos disponíveis.

### Armazenamento

<img src="https://cdn-icons-png.flaticon.com/512/8039/8039275.png " width="64">

.
- **Armazenamento Distribuído**: Utilize soluções de armazenamento distribuído como GlusterFS ou Ceph para gerenciar grandes volumes de dados.
- **Cache**: Implemente cache em serviços críticos utilizando Redis ou Memcached para reduzir a carga de banco de dados e melhorar o desempenho.

### Banco de Dados

<img src="   https://cdn-icons-png.flaticon.com/512/2906/2906274.png " width="64">

.
- **Replica Sets**: Configure réplicas de leitura para PostgreSQL para distribuir a carga de consultas.
- **Partitioning**: Utilize particionamento de tabelas para melhorar o desempenho de consultas em grandes volumes de dados.
- **Backup e Recuperação**: Configure políticas de backup frequentes e testes de recuperação para garantir a integridade dos dados.

### Sistema de Mensagens

<img src="https://cdn-icons-png.flaticon.com/512/16448/16448025.png " width="64" />


- **Cluster de RabbitMQ**: Configure um cluster de RabbitMQ para garantir alta disponibilidade e balanceamento de carga.
- **Monitoramento e Alertas**: Utilize ferramentas como RabbitMQ Management Plugin para monitorar e configurar alertas para filas e nós.

### Rede

<img src="https://cdn-icons-png.flaticon.com/512/16778/16778832.png" width="64" />

.
- **Balanceador de Carga**: Utilize balanceadores de carga (por exemplo, AWS ELB, Google Cloud Load Balancer) para distribuir o tráfego entre múltiplas instâncias de serviços.
- **CDN**: Utilize uma Rede de Distribuição de Conteúdo (CDN) para acelerar a entrega de conteúdo estático e melhorar a experiência do usuário final.

