# 关于MM


📫  您如果想联系我，可以直接发送邮件，邮箱地址.

💻

截止到 2024 年，有着超过 6 年的 AI Infra 的经验：


1. 在 AI 算法公司 [Unisound](https://www.s.com/) 负责 Atlas 超算平台的研发和运维，支持 NLP 以及 CV 模型训练。主要工作如下：
  - 开发大规模智能调度系统以优化多租户资源分配
  - 优化高性能分布式文件系统 Lustre 的性能
  - 构建多层缓存的云原生架构以加速 AI 模型训练


2. 在 Unisound 从事 8 Bit 训练及推理优化工作，落地模型在 NPU 以及 NVIDIA Edge Device 的优化工作。


1. 构建公有云大规模 AI 平台：
  - 通过 EKS （Elastic Kubernetes Service） 构建高性能、可伸缩的弹性离线训练平台。
  - 结合公有云对象存储以及加速器 GooseFS, 构建云上高性能缓存调度系统


2. 构建 FinOps 基础设施帮助公共云中的客户更轻松地管理优化云成本，提升云资源利用率：
  - 通过调度时以及重调度优化，通过高低优任务识别，智能弹性扩缩容。
  - 结合腾讯如意内核调度器优化以及可观测性，在保证服务质量的同时进行成本优化
  - 在内部云中大规模推出降低成本计划，通过资源的合理分配，提升资源利用率


1. 负责在 GCP 上构建 Serverless Inference 平台 [ModelZ](https://modelz.ai/)， 提供极致的冷启动优化模型服务推理服务：
  - 通过构建缓存模型服务、镜像预热等手段优化模型服务的冷启动时间
  - 引入 JuiceFS 构建高性能缓存调度系统，提升模型服务的性能


2. 负责整个 Cloud Team, 构建向量数据库 VectorChord 的云服务以及客户支持 [VectorChord Cloud](https://vectorchord.ai)：
  - 在 AWS 上构建基于 Postgres 的向量数据库，实现控制面、数据面分离，BYOC(Bring Your Own Cloud)、BYOD(Bring Your Own Data) 等功能
  - 引入云原生架构，实现 Postgres 存算分离、高可用、Backup、PITR(Point-In-Time Recovery)、In-Place Upgrade 等功能


### 开源项目

🌱 目前专注在 MLOps 以及 FinOps 领域,贡献了一些开源项目：
1. [fluid](https://github.com/fluid-cloudnative/fluid) Fluid, elastic data abstraction and acceleration for BigData/AI applications in cloud. (Project under CNCF)
2. [crane](https://github.com/gocrane/crane) Crane is a FinOps Platform for Cloud Resource Analytics and Economics in Kubernetes clusters. The goal is not only to help users to manage cloud cost easier but also ensure the quality of applications.
3. [crane-scheduler](https://github.com/gocrane/crane-scheduler) Crane scheduler is a Kubernetes scheduler which can schedule pod based on actual node load.
4. [creator](https://github.com/gocrane/creator) Creator is the brain of crane project, contains crane core algorithm module and evaluation module.
5. [openmodelz](https://github.com/tensorchord/openmodelz) One-click machine learning deployment (LLM, text-to-image and so on) at scale on any cluster (GCP, AWS, Lambda labs, your home lab, or even a single machine).
6. [clusternet](https://github.com/clusternet/clusternet) [CNCF Sandbox Project] Managing your Kubernetes clusters (including public, private, edge, etc.) as easily as visiting the Internet
7. [vectorchord](https://github.com/tensorchord/VectorChord) Scalable, fast, and disk-friendly vector search in Postgres, the successor of pgvecto.rs.

### 推荐博客

Type | Author/Company | Blog URL
--- | --- | ---
Infra |  [Chris Riccomini](https://cnr.sh/) | https://materializedview.io/
Infra | [Jack Vanlightly](https://jack-vanlightly.com/home) | https://jack-vanlightly.com/
Math and Science | 苏剑林 | https://kexue.fm/
AI Infra| Colfax | https://research.colfax-intl.com/blog/
Postgres | [Gabriele Bartolini](https://www.gabrielebartolini.it/about/) | https://www.gabrielebartolini.it/articles/
AI | [Sebastian Raschka](https://sebastianraschka.com/) | https://magazine.sebastianraschka.com/archive?sort=new
AI Algorithm | [Tom Yeh](https://www.linkedin.com/in/tom-yeh/) | https://www.byhand.ai/
AI Infra | [Chip Huyen](https://huyenchip.com/) | https://huyenchip.com/blog/

