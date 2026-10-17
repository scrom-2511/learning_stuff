# RabbitMQ vs BullMQ: Multi-Message & Notification Architecture

A practical comparison repository exploring the architectural differences between **RabbitMQ** (a dedicated Message Broker) and **BullMQ** (a Redis-backed Distributed Job/Task Queue), with hands-on code examples for multi-channel message dispatching (Email, Discord, Telegram).

---

## 🎯 The Core Motivation

When designing background processing pipelines in Node.js / TypeScript, a common question arises:

> *"Should I use BullMQ or RabbitMQ? Don't both put jobs/messages in a queue and have workers consume them?"*

While both handle asynchronous workloads, they are fundamentally different tools designed around different mental models:

- **RabbitMQ is an intelligent Message Broker (AMQP)** designed around **decoupled routing, exchanges, and message delivery protocols**.
- **BullMQ is a Job/Task Queue orchestrator** built on top of **Redis** designed around **job lifecycles, retries, delays, rate-limiting, and stateful task execution**.

This project implements the same high-level task in both systems to showcase how routing and publishing differ in practice:
> **Task**: When an event occurs (e.g., an order is placed or a broadcast event fires), send notifications across 3 channels: **Email**, **Discord**, and **Telegram**.

---

## 🧠 Architectural Mental Model

### BullMQ: Producer-Driven Multi-Queue Dispatch

BullMQ has **no concept of an exchange or smart message router**. Queues in BullMQ are distinct Redis data structures (lists, streams, sorted sets). 

If a producer wants to deliver a message to Email, Discord, and Telegram, the **producer itself must know about and push to all three queues explicitly**:

```
                  ┌───► [ email-queue ]     ───► Email Worker
                  │
[ Producer ] ──┼───► [ discord-queue ]   ───► Discord Worker
                  
                  └───► [ telegram-queue ]  ───► Telegram Worker
```

```ts
// BullMQ: Producer is tightly coupled to every destination queue
await Promise.all([
    enqueueEmailNoti(payload),
    enqueueDiscordNoti(payload),
    enqueueTelegramNoti(payload),
]);
```

---

### RabbitMQ: Broker-Driven Decoupled Routing (Exchanges & Bindings)

RabbitMQ decouples the producer from destination queues using **Exchanges**. 

Producers **never send messages directly to queues**. Instead:
1. Producer publishes **one single message** to an **Exchange**.
2. The Exchange inspects routing rules (bindings, routing keys, or headers).
3. The Broker routes duplicates of the message to all bound queues.
4. Services subscribe only to their own queue.

```
                                      ┌── [Binding] ──► [ email-queue ]    ──► Email Worker
                                      │
[ Event ] ──► ( notifications ) ──────┼── [Binding] ──► [ discord-queue ]  ──► Discord Worker
 (Producer)     Exchange (fanout)     │
                                      └── [Binding] ──► [ telegram-queue ] ──► Telegram Worker
```

```ts
// RabbitMQ: Producer publishes once; Broker handles duplication & distribution
await channel.publish("notifications", "", Buffer.from(msg));
```

The producer does not know or care whether there is 1 queue or 100 queues attached behind the `notifications` exchange.

---

## 🔀 RabbitMQ Routing Mechanisms (The Power of Exchanges)

RabbitMQ supports several exchange types depending on your routing needs:

| Exchange Type | Routing Logic | Typical Use Case |
| :--- | :--- | :--- |
| **`fanout`** *(Used in this repo)* | Ignores routing keys; duplicates and broadcasts the message to **all bound queues**. | Broadcast notifications, multi-service event broadcasting (Email + Discord + Telegram). |
| **`direct`** | Routes messages to queues whose binding key **exactly matches** the message's `routingKey`. | Targeted notifications (e.g., routing key `email` goes strictly to email queue). |
| **`topic`** | Wildcard pattern matching on routing keys (`*` matches one word, `#` matches zero or more words). | Scoped routing (e.g., `orders.europe.created`, `notifications.*.critical`). |
| **`headers`** | Uses message header attributes rather than routing keys. | Routing based on complex metadata headers (e.g., format, sender type). |

---

## ⚖️ Feature-by-Feature Comparison

| Capability | BullMQ (Redis-based) | RabbitMQ (AMQP Broker) |
| :--- | :--- | :--- |
| **Primary Category** | Distributed Job / Task Queue | Message Broker |
| **Underlying Engine** | Redis (RAM + RDB/AOF persistence) | Erlang BEAM / RabbitMQ Server (Disk/Memory) |
| **Message Routing** | None natively (producer pushes to each queue) | Native & Advanced (Direct, Fanout, Topic, Headers) |
| **Pub/Sub Fanout** | Manual (publish to N queues) | Native via Exchanges & Bindings |
| **Delayed & Scheduled Jobs** | Native and first-class (`delay`, repeatable cron) | Requires dead-letter exchange tricks or delayed exchange plugin |
| **Job Progress & States** | Native (`waiting`, `active`, `completed`, `failed`, `delayed`) | Not native (requires application-level tracking) |
| **Complex Workflows** | Native Flows (`FlowProducer` with parent/child dependencies) | Requires saga orchestration / choreography patterns |
| **Rate Limiting / Debounce** | Built-in per queue / per group | Requires external rate-limiter or custom consumer throttling |
| **Delivery Guarantees** | At-least-once (Redis durability dependent) | At-least-once / Exactly-once semantics via Quorum Queues & ACKs |
| **Ecosystem / Language** | Node.js / TypeScript (native), Python (`bullmq-python`) | Universal (Polyglot: Go, Java, Python, Node, C#, Rust, PHP) |

---

## 🚦 When Should You Use Which?

### Choose **BullMQ** when:
- **You are managing background "Jobs" or "Tasks"**, not just raw messages.
- You need built-in **delayed jobs, recurring cron tasks, or job progress tracking** (e.g., video processing: 20%, 50%, 100%).
- You need **parent-child job dependencies** (e.g., Run Job C only after Job A and Job B succeed).
- You already run **Redis** in your stack and don't want to operate a separate broker cluster.
- You need **rate limiting** (e.g., "Max 50 requests/second to OpenAI or Stripe API").

### Choose **RabbitMQ** when:
- **You need microservice-to-microservice communication** where services produce events and other services independently opt-in/subscribe.
- You need **flexible message routing** (e.g., routing by topic patterns like `analytics.*` or broadcasting via fanout).
- You are working in a **polyglot environment** (e.g., a Go service publishes an event, a Python ML worker consumes, and a Node.js API service consumes).
- You need enterprise-grade messaging topologies, high throughput message streaming, dead-letter routing, and fine-grained acknowledgement protocols.

---

## 📂 Project Structure

```bash
multiple-message-sender/
├── bullmq/                      # BullMQ Implementation
│   ├── compose.yaml             # Redis container
│   ├── src/
│   │   ├── index.ts             # Producer firing jobs to 3 distinct queues
│   │   ├── placeOrder.ts        # Order placement task dispatcher
│   │   └── services/            # Consumers for email, discord, telegram
│   └── package.json
│
├── rabbitmq/                    # RabbitMQ Implementation
│   ├── compose.yaml             # RabbitMQ container with management UI
│   ├── src/
│   │   ├── exchange.ts          # Defines fanout exchange 'notifications'
│   │   ├── setup.ts             # Declares queues (quorum) & binds to exchange
│   │   ├── producer.ts          # Publishes once to the exchange
│   │   ├── index.ts             # Trigger script
│   │   └── services/            # Consumers consuming bound queues
│   └── package.json
└── README.md
```

---

## 🚀 Running the Examples

### 1. BullMQ Example

```bash
cd bullmq

# 1. Start Redis
docker compose up -d

# 2. Install dependencies & run
npm install
npm run dev
```

### 2. RabbitMQ Example

```bash
cd rabbitmq

# 1. Start RabbitMQ (includes web UI at http://localhost:15672)
docker compose up -d

# 2. Install dependencies & run
npm install
npm run dev
```
