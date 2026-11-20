Feature: Empty GGML graphs
  @VT-14 @build
  Scenario: Prism empty-node admission matches the imported GGML CPU no-op behavior for empty copy graphs across every live storage type without accessing operand buffers
    Given unallocated empty DUP, CONT and CPY graphs for all live GGML types and every empty axis
    When the imported CPU reference and vTensor execute the graphs
    Then both succeed without allocating or accessing operand data
