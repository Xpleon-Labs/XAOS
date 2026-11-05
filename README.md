# XAOS

**Xpleon Agent Orchestration System**

In-process multi-agent runtime kernel for governed autonomous work.  
Agents propose tasks. Capability policy and the governance gate admit or deny. Admitted work runs under sandbox constraints. Decisions and outcomes append to a causal record.

XAOS is the **runtime** agentic Orchestration and management system

---

## Overview

XAOS runs a deterministic control loop over a fixed set of agent roles:

| Concern | Responsibility |
|---------|----------------|
| **Orchestration** | Tick loop, task queue, assignment, retries, dead-letter |
| **Proposal** | Ranked candidates (including ATE-assisted scoring where enabled) |
| **Governance** | Capability checks + gate evaluation before side effects |
| **Lifecycle** | Quarantine, heal eligibility, regeneration hooks |
| **Evidence** | Structured events and optional chain-linked vault records |
| **Learning** | Proposer-side adaptation from outcomes — not gate mutation |

External models may propose. XAOS decides whether anything runs.

---

## Architecture

```text
                 ┌──────────────────────────┐
                 │     Agent registry       │
                 │  roles · queues · health │
                 └────────────┬─────────────┘
                              │
                              ▼
                 ┌──────────────────────────┐
                 │      Orchestrator        │
                 │   tick · schedule · DLQ  │
                 └────────────┬─────────────┘
                              │
              propose ────────┤
                              ▼
                 ┌──────────────────────────┐
                 │  Capability + domain     │
                 │  policy (allow / deny)   │
                 └────────────┬─────────────┘
                              │
                              ▼
                 ┌──────────────────────────┐
                 │     Governance gate      │
                 │  rules · score · deny    │
                 └────────────┬─────────────┘
                     admit │ deny
              ┌────────────┴────────────┐
              ▼                         ▼
     ┌─────────────────┐       ┌─────────────────┐
     │    Execution    │       │   Dead-letter   │
     │ sandbox / tools │       │   + reason code │
     └────────┬────────┘       └────────┬────────┘
              │                         │
              └────────────┬────────────┘
                           ▼
                 ┌──────────────────────────┐
                 │  Causal event log / WAL  │
                 │  (optional ElderVault)   │
                 └──────────────────────────┘


Invariants
No tool path bypasses the gate.
Unknown or unmatched external actions fail closed.
Outcome data may update proposer weights; it does not rewrite the rulebook.
Replay and audit rest on ordered, content-linked records — not chat logs alone.

**Module**                                                **Function**
Orchestrator                           Synchronous tick cycle: dequeue, assign, govern, execute or dead-letter"
Agent registry                         Named roles, queue depth, status, notes, quarantine flags"
ATE-assisted proposer                  Feature-scored candidates for ranking (proposal plane only)
Capability policy                      equired permissions and domain constraints per task kind
Governance gate                        Explicit admit / deny / halt with recorded rule identity
Lifecycle                              Quarantine, heal, and regeneration eligibility after failures"
Workflow synthesizer                   Draft multi-step workflows subject to approval / gate
Execution path                         Allowlisted shell, jailed paths, constrained HTTP where enabled"
Learning store                         Append-only experience (JSONL or equivalent) for proposer adaptation
Evidence plane                         Event emission; optional fixed-width causal vault records

**Security model** 
**Separation of planes : proposal ≠ admission ≠ actuation.
Fail closed : missing capability, unknown action, or gate miss → deny / dead-letter.
Sandbox defaults : no open shell, no open HTTP; allowlists only.
Evidence : decisions and outcomes are ordered records suitable for review.
Learning bound : adaptation does not enlarge authority or edit the rulebook.

License Proprietary- Xpleon Labs.

Contact: 
Xpleon Labs
https://xpleonlabs.com
contact@xpleonlabs.com
