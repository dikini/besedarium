# Task 2.4.1: Integration Test Overhaul Plan

## Current State Analysis

All existing integration tests in `/tests/` use the old protocol API that was removed during legacy cleanup:
- `TStart`, `TSend`, `TChoice` etc. (old global protocol types)
- `ProtocolLabel` trait (replaced with our `MsgLbl` trait)
- Channel types like `Http`, `Mqtt` (replaced with our `CommMetadata` system)
- Legacy test infrastructure

## Decision: Complete Replacement

**Reasoning:**
1. **API Incompatibility**: All existing tests use deprecated APIs that no longer exist
2. **Architecture Change**: Tests use old single-channel model vs. our multi-channel `CommMetadata` approach
3. **Type System Evolution**: Tests use old trait system vs. our new foundation types
4. **Clean Slate Benefit**: New tests can demonstrate best practices from the start

## Integration Test Strategy

### Task 2.4.1: Remove Legacy Tests ✅
- Remove all `.disabled` test files (they're unsalvageable)  
- Keep `trybuild.rs` infrastructure for compile-failure tests
- Create clean slate for new integration tests

### Task 2.4.2: Multi-Party Protocol Examples
- Client-Server handshake using `TChanSend`/`TChanRecv`
- Three-party authentication protocol
- Publish-Subscribe with broker coordination

### Task 2.4.3: Complex Data Serialization
- JSON message protocols
- Binary data streaming
- Mixed message type scenarios

### Task 2.4.4: Async Runtime Integration  
- Tokio integration examples
- Real channel communication
- Error handling and recovery

## Implementation Approach

1. **Clean removal** of legacy disabled tests
2. **Modern test structure** using our foundation types
3. **Realistic protocols** that showcase library capabilities
4. **Progressive complexity** from simple to advanced scenarios
