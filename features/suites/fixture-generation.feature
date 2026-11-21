Feature: fixture-generation

  @VT-35 @build
  Scenario: Prism generates all matrix fixture dimensions layouts and input words matching the complete frozen prior oracle corpus while GGML independently computes every expected result
    Given the fixture VT-35 needs
    When the suite exercises VT-35
    Then Prism generates all matrix fixture dimensions layouts and input words matching the complete frozen prior oracle corpus while GGML independently computes every expected result
