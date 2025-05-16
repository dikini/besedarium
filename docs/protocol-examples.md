# Protocol Examples: Review and Modeling Analysis

## Introduction

This document reviews the protocol examples from the classic session types literature, as found in
`protocol-examples.md`, and analyzes how they can be modeled using the current Besedarium library.
It also discusses the challenges and limitations encountered, especially around recursion, choices,
and protocol projection. The aim is to provide a practical, hands-on perspective for protocol
designers and implementers.

---

## 1. Customer-Agency Protocols

### 1.1. Simple Protocol (No Recursion)

**Protocol Steps:**

1. Customer sends an order to the agency (e.g., a travel destination).
2. Agency replies with a quote (a price).
3. Customer decides to accept or reject the offer:
   - If accepted: Customer sends address, agency sends confirmation date, protocol ends.
   - If rejected: Protocol ends immediately.

**Global Protocol (from the paper):**

```ignore
Customer → Agency : order(Hawaii).
Agency → Customer : quote(nat).
Customer → Agency : {
    accept(bool).
        Customer → Agency : address(nat).
        Agency → Customer : date(nat).end,
    reject(bool).end
}
```

**Modeling in Besedarium:**

- This protocol can be modeled using a sequence of `TInteract` (for messages) and `TChoice` (for
the accept/reject decision).
- Each branch of the choice is a sequence of further interactions, ending with `TEnd`.

**Global Protocol (Besedarium API):**

```rust
type SimpleGlobal =
    TInteract<
        Http,
        EmptyLabel,
        Customer,
        Order<Hawaii>,
        TInteract<
            Http,
            EmptyLabel,
            Agency,
            Quote<Nat>,
            TChoice<
                Http,
                EmptyLabel,
                // accept branch
                TInteract<
                    Http,
                    EmptyLabel,
                    Customer,
                    Accept<Bool>,
                    TInteract<
                        Http,
                        EmptyLabel,
                        Customer,
                        Address<Nat>,
                        TInteract<
                            Http,
                            EmptyLabel,
                            Agency,
                            Date<Nat>,
                            TEnd<Http>
                        >
                    >
                >,
                // reject branch
                TEnd<Http>
            >
        >
    >;
```

**Projection to Local Types:**

- Each role (Customer, Agency) gets a local protocol, where choices and message directions are
preserved.
- The choice is made by the Customer, and the Agency offers the corresponding branches.

---

### 1.2. Protocol with Recursion (Retry)

**Global Protocol:**

```ignore
rec {
    Customer → Agency : order(place).
    Agency → Customer : quote(nat).
    Customer → Agency : {
        accept(bool).
            Customer → Agency : address(nat).
            Agency → Customer : date(nat).end,
        retry.
            Customer → Agency : retry
        reject(bool).end
    }
}
```

**Key Point:**

- The `retry` branch allows the protocol to loop back and start over.
- This is a classic use of recursion in session types.

**Modeling in Besedarium:**

- Recursion is modeled with `TRec` (for the recursive block).
- The `retry` branch should loop back to the start of the recursion.
- The current library uses a simple `TRec`, but does not have explicit recursion variables or a way
to "break out" of recursion in a type-safe way.

**Global Protocol (Besedarium API):**

```rust
type RetryGlobal =
    TRec<
        Http,
        RecursionLabel<"retry_loop">,
        TInteract<
            Http,
            EmptyLabel,
            Customer,
            Order<Place>,
            TInteract<
                Http,
                EmptyLabel,
                Agency,
                Quote<Nat>,
                TChoice<
                    Http,
                    EmptyLabel,
                    // accept
                    TInteract<Http, EmptyLabel, Customer, Accept<Bool>, /*...*/> ,
                    TChoice<
                        Http,
                        EmptyLabel,
                        // retry branch loops to recursion point
                        TInteract<Http, EmptyLabel, Customer, Retry, /* loops via TRec */>,
                        TEnd<Http>
                    >
                >
            >
        >
    >;
```

**Projection to Local Types:**

- The Customer and Agency both see the recursion, but the decision to retry is made by the Customer
and communicated explicitly.
- This ensures both roles are synchronized and prevents protocol divergence.

**Example Local Projection (Customer):**

```rust
// Customer sees recursion loop with accept, retry, or reject
type CustomerLocalRetry =
    EpSend<
        Http,
        Customer,
        Order<Place>,
        EpRecv<
            Http,
            Customer,
            Quote<Nat>,
            EpChoice<
                Http,
                Customer,
                // accept branch
                EpSend<
                    Http,
                    Customer,
                    Address<Nat>,
                    EpRecv<
                        Http,
                        Customer,
                        Date<Nat>,
                        EpEnd<Http, Customer>
                    >
                >,
                // nested choice: retry or reject
                EpChoice<
                    Http,
                    Customer,
                    // retry: implicit recursion via TRec
                    EpSend<Http, Customer, Retry, /* loops via TRec */>,
                    // reject: end
                    EpEnd<Http, Customer>
                >
            >
        >
    >;
```

**Example Local Projection (Agency):**

```rust
// Agency offers branches under recursion: accept, retry, or reject
type AgencyLocalRetry =
    EpRecv<
        Http,
        Agency,
        Order<Place>,
        EpSend<
            Http,
            Agency,
            Quote<Nat>,
            EpChoice<
                Http,
                Agency,
                // accept branch
                EpRecv<
                    Http,
                    Agency,
                    Address<Nat>,
                    EpSend<
                        Http,
                        Agency,
                        Date<Nat>,
                        EpEnd<Http, Agency>
                    >
                >,
                // nested choice: retry or reject
                EpChoice<
                    Http,
                    Agency,
                    // retry: implicit recursion via TRec
                    EpRecv<Http, Agency, Retry, /* loops via TRec */>,
                    EpEnd<Http, Agency>
                >
            >
        >
    >;
```

---

## 2. Web Service with Proxy

**Protocol Description:**

- A client, proxy, and web service interact.
- The client sends a request to the proxy.
- The proxy chooses to either forward the request or audit it.
- In the forward case, the web service replies to the client.
- In the audit case, the web service sends details to the proxy, which then resumes the session,
and finally the web service replies to the client.

**Global Protocol:**

```ignore
Client → Proxy : request {
    forward.
        Proxy → Web Service : forward.
        Web Service → Client : reply
    audit.
        Proxy → Web Service : audit.
        Web Service → Proxy : details.
        Proxy → Web Service : resume.
        Web Service → Client : reply
}
```

**Modeling in Besedarium:**

- The protocol is modeled as a sequence of `TInteract` and a `TChoice` (for the proxy's decision).
- Each branch is a sequence of further interactions, ending with `TEnd`.

**Projection to Local Types:**

- Each role gets a local protocol, with the proxy making the choice and the others offering the
corresponding branches.

### Example Local Projection (Client)

```rust
// Client sends request then offers two symmetric reply branches
type ClientLocal =
    EpSend<
        Http,
        Client,
        Request,
        EpChoice<
            Http,
            Client,
            EpRecv<Http, Client, Reply, EpEnd<Http, Client>>,
            EpRecv<Http, Client, Reply, EpEnd<Http, Client>>
        >
    >;
```

### Example Local Projection (Proxy)

```rust
// Proxy receives request, chooses forward or audit, then dispatches messages
type ProxyLocal =
    EpRecv<
        Http,
        Proxy,
        Request,
        EpChoice<
            Http,
            Proxy,
            // forward branch
            EpSend<
                Http,
                Proxy,
                Forward,
                EpRecv<
                    Http,
                    Proxy,
                    Reply,
                    EpSend<Http, Proxy, Reply, EpEnd<Http, Proxy>>
                >
            >,
            // audit branch
            EpSend<
                Http,
                Proxy,
                Audit,
                EpRecv<
                    Http,
                    Proxy,
                    Details,
                    EpSend<
                        Http,
                        Proxy,
                        Resume,
                        EpRecv<
                            Http,
                            Proxy,
                            Reply,
                            EpSend<Http, Proxy, Reply, EpEnd<Http, Proxy>>
                        >
                    >
                >
            >
        >
    >;
```

### Example Local Projection (Web Service)

```rust
// Web Service offers forward or audit handling
type WebServiceLocal =
    EpChoice<
        Http,
        WebService,
        // forward branch
        EpRecv<
            Http,
            WebService,
            Forward,
            EpSend<Http, WebService, Reply, EpEnd<Http, WebService>>
        >,
        // audit branch
        EpRecv<
            Http,
            WebService,
            Audit,
            EpSend<
                Http,
                WebService,
                Details,
                EpRecv<
                    Http,
                    WebService,
                    Resume,
                    EpSend<Http, WebService, Reply, EpEnd<Http, WebService>>
                >
            >
        >
    >;
```

---

## 3. Modeling Challenges and Limitations

### 3.1. Recursion and Control Flow

- The current Besedarium library models recursion with `TRec`, but lacks explicit recursion
variables (like Mu/De Bruijn indices).
- This means you cannot precisely control where to break out of recursion or refer to a specific
recursion point.
- Infinite loops are possible if recursion is not properly controlled by explicit protocol actions.

### 3.2. Synchronization of Recursion

- Recursion control (break/continue) must always be driven by explicit protocol messages or
choices, not by local-only control flow.
- If one role decides to break/continue locally, but the others do not, the protocol can diverge or
deadlock.
- The recommended approach is to always synchronize recursion control via explicit messages, as
shown in the examples above.

### 3.3. Labels and Choices

- Adding labels to choices, recursion, and ends improves clarity and helps with code generation and
projection.
- However, labels alone do not synchronize protocol control flow; explicit messages are still
required.

### 3.4. Projection Safety

- The projection algorithm must ensure that all roles are synchronized, especially around choices
and recursion.
- All break/continue decisions must be protocol-driven, not local-only.

---

## 4. References

- [A Very Gentle Introduction to Multiparty Session
Types][msession-types-intro]
- [Comprehensive Multiparty Session Types](https://arxiv.org/pdf/1902.00544)

[msession-types-intro]: http://mrg.doc.ic.ac.uk/publications/a-very-gentle-introduction-to-multiparty-session-types/main.pdf

---

## 5. Discussion: Recursion, Labels, and Scoping

### 5.1. Mu/Var vs. Labelled Rec/Break

A key insight from session type theory is that labelled recursion (e.g., `Rec<"loop"> ...
Break<"loop">`) is functionally equivalent to the classic `Mu(X) ... Var(X)` approach, as long as
labels (or variables) are unique and in scope. Both allow you to define a recursion point and refer
back to it, enabling looping and structured control flow in protocols.

#### Example: Ping-Pong with Flat Labels

```rust
Rec<"main_loop",
    Interact<Alice, Bob, Ping,
        Interact<Bob, Alice, Pong,
            Choice<Alice, (
                ("again", Break<"main_loop">),
                ("stop", End)
            )>
        >
    >
>
```

- Here, `Rec<"main_loop">` introduces a recursion point with a globally unique label.
- `Break<"main_loop">` refers unambiguously to that point, looping back.
- This is equivalent to `Mu(X) ... Var(X)` in classic session types.

### 5.2. Flat Namespace: Simplicity and Limitations

By enforcing a single global namespace for recursion labels (i.e., all labels must be unique within
a protocol), we:

- **Avoid Scoping Complexity:** No need for nested scopes, shadowing, or stack-based resolution.
Label lookup is always global.
- **Simplify Implementation:** Projection, type-checking, and code generation are
straightforward—just match labels globally.
- **Catch Errors Early:** Duplicate or ambiguous labels are caught at protocol definition time.

#### Limitations

- **No Mutual Recursion:** You cannot have two recursion points with the same label, so mutual
recursion (where two or more recursion points refer to each other) is not possible.
- **Flat Namespace:** All labels must be unique, which could be a minor inconvenience in very large
or generated protocols.
- **Expressiveness:** For most practical protocols, this is not a problem, but it does restrict the
theoretical expressiveness compared to full Mu/Var with scoping.

#### Example: What You Cannot Do

Suppose you want two mutually recursive blocks:

```ignore
Rec<"A",
    ... Break<"B"> ...
>
Rec<"B",
    ... Break<"A"> ...
>
```

With a flat namespace, you cannot have both "A" and "B" in scope at the same time, so this pattern
is not supported.

### 5.3. Higher-Level Protocol Languages

The flat label approach can serve as a substrate for higher-level protocol languages or libraries.
For example:

- A macro or code generator could manage unique label generation and simulate mutual recursion by
flattening or inlining protocol fragments.
- A more advanced protocol language could introduce scoped labels or variables, compiling down to
the flat-label substrate for execution or type-checking.

### 5.4. Design Guidance

- **Start Simple:** Use a flat, globally unique label namespace for recursion. This covers the vast
majority of real-world protocols and keeps the system easy to reason about.
- **Document Limitations:** Be explicit in documentation and error messages about the lack of
mutual recursion and the requirement for unique labels.
- **Plan for Extensibility:** If future needs require more expressiveness (e.g., mutual recursion),
consider layering a higher-level language or macro system on top of the flat-label core.

### 5.5. Summary Table

| Approach                | Pros                        | Cons                        | Use Case
              |
|-------------------------|-----------------------------|-----------------------------|-------------
--------------|
| Flat global labels      | Simple, easy to implement   | No mutual recursion         | Most
real-world protocols |
| Scoped Mu/Var           | Most expressive             | Complex, harder to use      |
Advanced/academic         |
| Macro/codegen           | User-friendly, flexible     | Tooling required            |
Large/generated protocols |

---

*This section was added to clarify the design trade-offs around recursion, labels, and scoping in
Besedarium. It is intended to guide both users and implementers in making informed, practical
choices.*

---

## 6. Mutual Recursion via Par and Rec: Options, Dangers, and Caveats

### 6.1. Modeling Mutual Recursion with Par and Rec

It is possible to encode certain forms of mutual recursion by combining `Par` (parallel
composition) and `Rec` (recursion), provided the restriction that Par branches must have disjoint
sets of roles is relaxed. In this approach:

- Each `Rec` block represents a protocol state or phase.
- `Par` allows these states to be "active" in parallel.
- Shared roles can coordinate transitions between these states by sending/receiving messages that
trigger a jump from one Rec block to another.

#### Example: Two-State Mutual Recursion

```ignore
Par(
  Rec<"A",
    ... Choice { toB: ...Break<"B">... } ...
  >,
  Rec<"B",
    ... Choice { toA: ...Break<"A">... } ...
  >
)
```

Here, both Rec blocks are live, and transitions between A and B are coordinated by explicit
protocol actions.

### 6.2. Dangers and Caveats

#### a. **Synchronization Complexity**

- When roles are shared between Par branches, transitions between states must be carefully
synchronized by explicit messages.
- If one role transitions but another does not, the protocol can deadlock or diverge.
- The projection algorithm and runtime must ensure that all roles agree on the current state.

#### b. **State Explosion and Reasoning**

- Multiple Rec blocks running in parallel can lead to a state explosion, making the protocol harder
to reason about, verify, and maintain.
- Deadlock-freedom and progress become much harder to check, as the number of possible
interleavings increases.

#### c. **Expressiveness vs. Safety**

- While this approach increases expressiveness (allowing more complex, interleaved, or stateful
protocols), it also increases the risk of subtle bugs, such as unsynchronized transitions,
livelocks, or unreachable states.
- The lack of disjointness means that the same role may have to "choose" between multiple possible
actions at the same time, which can be ambiguous or ill-defined.

#### d. **Tooling and Implementation Burden**

- Projection, type-checking, and code generation become significantly more complex.
- Runtime implementations must track and synchronize state across all roles, which may require
additional protocol messages or coordination logic.

### 6.3. Exploring the Options

#### Option 1: **Keep Disjointness Restriction**

- Simpler, safer, and easier to reason about.
- No mutual recursion, but protocols are easier to verify and implement.

#### Option 2: **Relax Disjointness for Advanced Users**

- Allows mutual recursion and more expressive protocols.
- Must be accompanied by strong warnings, advanced static analysis, and possibly runtime checks to
prevent deadlocks and divergence.
- Best suited for protocol designers who understand the risks and are willing to invest in careful
design and verification.

#### Option 3: **Higher-Level Abstractions**

- Provide macros, code generators, or higher-level protocol languages that can safely encode mutual
recursion patterns, compiling down to safe, well-formed Par/Rec combinations.
- This can hide complexity from most users while still allowing advanced expressiveness when needed.

### 6.4. Summary Table

| Option                        | Pros                        | Cons                        | Use
Case                  |
|-------------------------------|-----------------------------|-----------------------------|-------
--------------------|
| Disjoint Par (default)        | Simple, safe, verifiable    | No mutual recursion         | Most
protocols            |
| Par+Rec, shared roles         | Expressive, flexible        | Complex, risky, error-prone |
Advanced protocols        |
| Macro/codegen abstraction     | User-friendly, safe         | Tooling required            |
Large/generated protocols |

### 6.5. Guidance

- **Default to safety:** Keep the disjointness restriction unless there is a compelling need for
mutual recursion.
- **Document risks:** If relaxing the restriction, clearly document the dangers and require
explicit opt-in.
- **Invest in tooling:** If supporting advanced patterns, provide static analysis and runtime
checks to help users avoid common pitfalls.
- **Encourage explicit synchronization:** Always require that transitions between states are driven
by explicit protocol actions, not by local or implicit control flow.

---

*This section documents the options, dangers, and caveats of modeling mutual recursion via Par and
Rec, to guide protocol designers in making informed, safe choices.*

---

*Prepared by GitHub Copilot, 12 May 2025*
