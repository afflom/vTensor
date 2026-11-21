Feature: Certified graph fixtures
  @VT-35 @build
  Scenario: Prism generates all matrix fixture dimensions layouts and input words matching the complete frozen prior oracle corpus while GGML independently computes every expected result
    Given the complete frozen records of 1280 prior matrix cases including source layouts and exceptional input words
    When the certified Prism fixture model generates every case and the imported GGML constructors bind it
    Then generated and observed fixture records match the frozen corpus byte for byte
    And the imported GGML CPU computes every expected numerical result independently
    And changed source words missing cases and wrong field bindings are refused
