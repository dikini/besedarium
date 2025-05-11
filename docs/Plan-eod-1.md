# Updated Plan: Next Steps for Session Types Library

---

## 1. Disjointness Checks (**Highest Priority**)
- [x] **Implement type-level enforcement of disjointness for `TPar` branches**
    - [x] **Step 1: Role Extraction**
        - [x] Define a `RolesOf` trait for all session combinators to extract a type-level list of roles used in a session.
        - [x] Implement `RolesOf` for `TInteract`, `TChoice`, `TPar`, `TRec`, and `TEnd`.
    - [x] **Step 2: Disjointness Traits**dd documentation and examples -
        - [x] Define a type-level list trait `Contains<X>` and `Disjoint<A, B>` to check if two role lists are disjoint.
        - [x] Implement supporting traits like `NotContains<X>`, `NotSame<X>`, and `NotEqual<X>`.
    - [x] **Step 3: Branding TPar**
        - [x] Add a type-level boolean parameter to `TPar` (e.g., `TPar<L, R, IsDisjoint>`) to indicate if disjointness is guaranteed.
        - [x] By default, mark as `False` (unchecked); after explicit check, rebrand as `True`.
    - [x] **Step 4: Ergonomic Compile-Time Assertions**
        - [x] Provide a trait or macro (e.g., `AssertDisjoint<TPar>`) to check and rebrand a `TPar` as disjoint.
        - [x] Update the `assert_disjoint!` macro to use the new traits.
    - [x] **Step 5: Documentation and Examples**
        - [x] Add doc comments and usage examples for all new traits and macros.
        - [x] Add compile-time tests for both successful and failing disjointness checks.

---

## 2. Error Messages (**Very Important**)
- [x] **Improve developer experience with clear compile-time errors**
    - [x] Use custom traits/macros for better error messages
    - [x] Document common errors and their resolutions

---

## 3. Testing and Examples (**Important**) 
- [x] **Add comprehensive compile-time tests and usage examples**
    - [x] Edge cases: deeply nested combinators, recursive protocols
    - [x] Macro ergonomics: single-branch
    - [x] Disjointness/type equality: complex/nested/n-ary cases (positive/compiling cases)
    - [x] **Empty protocols (should fail) are now fully automated as trybuild compile-fail tests.**
    - [x] Negative tests: compile-fail for incorrect usage (mixed IO, duplicate roles, etc.) are fully automated and always run via trybuild.
    - [x] **Macro ergonomics: trailing commas and whitespace are now tested and covered by trybuild.**
    - [x] **All doc-tests/examples in crate-level documentation are now registered and passing.**
- [x] **Add realistic and illustrative example protocols**
    - [x] Client-server handshake (e.g., HTTP request/response)
    - [x] Publish/subscribe (e.g., MQTT or event bus)
    - [x] Multi-party workflow (client, server, broker, worker)
    - [x] Recursive/streaming protocol
    - [x] Protocols using TChoice for branching (e.g., login vs. register)
    - [x] Protocols using TPar for concurrency (e.g., parallel downloads)
    - [x] Protocols using TRec for loops/repeats
    - [x] Protocols using Mixed marker for informational use
    - [x] **Protocols are now in separate files in `tests/protocols/` for discoverability and documentation.**

---

## 4. Documentation (**Important**)
- [ ] **Comprehensive documentation**
    - [ ] Module-level and item-level doc comments
    - [ ] Design patterns, safety guarantees, and usage examples
    - [ ] Seek AI assistance for drafting and refining documentation
    - [x] **README.md added with approachable, non-academic documentation and mermaid diagrams for all main protocol examples.**
    - [x] **README.md is now included as module-level documentation via `#![doc = include_str!("../README.md")]` for docs.rs and cargo doc.**

---

## 5. Library Structure and Modularization (**Extras**)
- [ ] **Review and improve library structure**
    - [ ] Keep in a single file for now
    - [ ] After API is stable, review for further modularization
        - [ ] Make constructors private to prevent invalid protocols
        - [ ] Separate core traits, combinators, and utilities into modules if needed

---

## 6. Macros for Ergonomics and Safety (**New Section**)
- [ ] **Add macros to improve ergonomics and code quality**
    - [ ] **Type-level list construction macro**
        - [ ] `tlist!` macro for building type-level lists
    - [ ] **N-ary choice/par construction macros**
        - [ ] `tchoice!` and `tpar!` macros for n-ary combinators
    - [ ] **Compile-time assertion macros**
        - [ ] `assert_type_eq!` for type equality
        - [ ] `assert_disjoint!` for disjointness checks
    - [ ] **Attribute/proc macros for protocol definitions (future/optional)**
        - [ ] Explore attribute macros for protocol DSLs or runtime representations
    - [ ] **Helper macros for role extraction/disjointness**
        - [ ] Macros to extract roles or assert disjointness
    - [ ] **Document macros with examples.**

---

## 7. Additional Considerations
- [ ] **Duality:** Consider adding duality checks for endpoint compatibility in the future
- [ ] **Labelled Ends:** Plan for labelled ends and plug traits if advanced recursion/scatter-gather is needed
- [ ] **Multiparty Extensions:** Plan for global type/projected type machinery if targeting multiparty protocols


# End-of-Day Codebase Snapshot: main.rs

## Macros

- `tlist!`: Builds type-level lists (`Cons`, `Nil`) for ergonomic n-ary combinator construction.
- `tchoice!`: Builds n-ary choices from a type-level list using `ToTChoice`.
- `tpar!`: Builds n-ary parallel compositions from a type-level list using `ToTPar`.
- `assert_type_eq!`: Compile-time assertion for type equality.
- `assert_disjoint!`: Compile-time assertion for type-level disjointness (to be implemented with disjointness traits).

## Core Type-Level Machinery

- **Type-level lists:** `Nil`, `Cons`
- **Roles:** `TClient`, `TServer`, `TBroker`, `TWorker` (all implement `TRole`)
- **Session combinators:**
  - `TEnd`: End of session
  - `TInteract<R, H, T>`: Interaction for role `R` with message `H`, then continue as `T`
  - `TRec<S>`: Recursive session type
  - `TChoice<L, R>`: Binary choice between two branches
  - `TPar<L, R>`: Binary parallel composition of two threads

## Traits

- `TSession`: Core trait for session types, with `Compose` and `IS_EMPTY`
- `ToTChoice`: Maps a type-level list to nested `TChoice`
- `ToTPar`: Maps a type-level list to nested `TPar`
- `Sealed`: Prevents external implementation of core traits

## Example Protocols

- **Sequential composition:**  
  `type Chain = Compose<Compose<ClientServer, ServerBroker>, TRec<BrokerWorker>>;`
- **Binary choice:**  
  `type ChoiceSession = TChoice<TInteract<TClient, Message, TEnd>, TInteract<TClient, Publish, TEnd>>;`
- **4-way choice (manual):**  
  `type PlainFourWayChoice = TChoice<...>;`
- **4-way choice (n-ary):**  
  `type FourWayChoice = <NaryChoice as ToTChoice>::Output;`

## Compile-Time Tests

- `_assert_choices_equal(_: FourWayChoice, _: PlainFourWayChoice)`:  
  Compile-time check that n-ary and manual 4-way choices are the same type.

## Main Function

- Prints a placeholder and calls `_assert_choices_equal` with `core::panic!()` to ensure type-checking only.

---

**Notes:**
- Disjointness checking for `TPar` is planned but not yet implemented.
- Macros are ready for ergonomic protocol construction and compile-time assertions.
- The codebase is compact, modular, and ready for further extension.

---

*Snapshot and summary by GitHub Copilot, 2025.*
*Plan drafted by user dikini and GitHub Copilot, 2025.*