Feature: GGML repeat-back reductions
  @VT-20 @build
  Scenario: Prism models four-axis REPEAT_BACK initialization and accumulation order and Verdant execution matches the imported GGML F32 CPU oracle across strided and empty tensors
    Given every four-axis repetition combination and order-sensitive finite input values
    When vTensor initializes and folds through Prism plans and Verdant addition
    Then the complete output matches the imported CPU graph and padding is preserved
    And invalid extents and missing input descriptors fail before writes
