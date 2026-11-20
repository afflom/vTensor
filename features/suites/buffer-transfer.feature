Feature: buffer-transfer

  @VT-09 @build
  Scenario: Prism transfer admission matches upstream GGML set and get guards and rejects overflowing ranges
    Given a live tensor with observed buffer and allocation state
    When Prism admits or refuses set and get requests including views
    Then its decision matches the pinned upstream guards for representable ranges
    And zero-byte transfers require a buffer but need no data allocation
    And overflowing ranges are refused without wrapping
