Feature: relative-position-add

  @VT-39 @build
  Scenario: Prism models copied and in-place GGML ADD_REL_POS event ordering and Verdant matches the imported CPU oracle with atomic descriptor refusals
    Given the pinned GGML constructors CPU implementation and unchanged imported test_case evaluation machinery
    When copied and in-place ADD_REL_POS graphs execute through Prism and Verdant
    Then every required graph matches the actual CPU with exact finite results
    And invalid descriptors and unsafe aliases fail before writes
    And missing cases and altered execution bindings are rejected
