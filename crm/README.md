# CRM 系统架构

把四层服务拆开后，各自只负责“一件事”，整个 CRM 才能做到高内聚、低耦合、易扩容。下面按 数据流顺序 说明：

## 1. User Stat Service（用户行为事实表）
输入：业务系统（播放、点击、登录…）实时或批量写行为事件
存储：ClickHouse / DuckDB 等列式 OLAP，5 M~千万级数据
输出：提供 用户宽表 user_stats（邮箱、姓名、最后访问、观看进度、各阶段计数器、最近观看列表…）

职责：
✅ 行为事件落地
✅ 预聚合/物化视图生成宽表
✅ 对外 RPC：query(QueryRequest) → 返回宽行数据

## 2. Metadata Service（内容/商品元数据）
输入：CMS、媒资、商品中心
存储：Postgres / MySQL + 缓存
输出：视频/商品基础信息（标题、标签、上下架时间、价格…）

职责：
✅ 内容维度表维护
✅ 标签体系、分类树
✅ 对外 RPC：materialize(MaterializeRequest) → 返回内容详情

## 3. Send Service（发送通道 & 持久化队列）
输入：上游 CRM 产生的“待触达任务”
存储：持久化消息队列（Kafka / RocketMQ /自建 MPSC）
通道：Email、短信、App Push、站内信

职责：
✅ 任务去重、削峰、重试、死信
✅ 多通道并行下发
✅ 回执/点击事件回流给 User Stat Service
✅ 对外流式 RPC：send(stream SendRequest) → 返回发送结果流

## 4. CRM Service（策略编排 & 工作流引擎）
输入：
定时扫描 User Stat（过滤“看过但未开始”（viewed_but_not_started > 0））
调用 Metadata 补齐内容标题/封面

决策：
召回（Recall）：沉默 7 天用户 → 发“你可能感兴趣”
提醒（Remind）：开始未看完 → 发“继续观看”
通用营销（General）：新品、生日、会员到期
输出：生成“发送任务”写入 Send Service 队列

职责：
✅ 规则引擎 / AB 实验
✅ 频控、模板渲染
✅ 实时+离线双模式（Flink / 定时任务）
✅ 对外 RPC：recall / remind / general

## 数据流一句话总结
行为事件 → User Stat → CRM 策略 → Metadata 补齐 → Send 队列 → 通道下发 → 回执再回流 User Stat（闭环）。
这样四层各司其职，可独立扩容、独立部署，也方便后续接入实时 Flink、离线 Spark 做更复杂的用户关怀与商业分析。
