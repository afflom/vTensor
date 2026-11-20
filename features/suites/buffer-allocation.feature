Feature: GGML tensor placement

  @VT-10 @build
  Scenario: Prism tensor placement matches the imported GGML linear allocator for aligned buffers and refuses invalid alignment, exhausted capacity, and arithmetic overflow
    Given the pinned upstream GGML linear allocator
    When Prism places tensors in aligned buffers at boundary capacities
    Then placement offsets and padding match the allocator
    And invalid alignment, exhausted capacity, and arithmetic overflow are refused
