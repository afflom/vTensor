Feature: GGML storage copy graphs
  @VT-15 @build
  Scenario: Prism plans same-type DUP, CONT and CPY across every live GGML storage type and Verdant moves the admitted blocks with upstream-equivalent logical order and preserved padding
    Given every live storage type with contiguous, padded and reshaped storage
    When vTensor copies the tensors using Prism block plans and Verdant execution
    Then logical bytes match the imported CPU copy and contiguous reference graph
    And output padding remains unchanged
