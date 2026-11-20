Feature: GGML diagonal construction
  @VT-21 @build
  Scenario: Prism defines batched DIAG placement and Verdant preserves diagonal encodings and positive-zero off-diagonal values for every type supported by the imported GGML CPU oracle
    Given F32 F16 and BF16 vectors with batches, strides and empty dimensions
    When vTensor executes the Prism DIAG plan through Verdant storage transfers
    Then the complete output matches the imported CPU graph and preserves padding
    And invalid shapes and missing source descriptors fail before writes
